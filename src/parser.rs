use std::{
    fs::File,
    io::{self, Read},
};

#[derive(Debug, PartialEq, Eq)]
enum InstructionType {
    AInstruction,
    CInstruction,
    LInstruction,
}

impl InstructionType {
    fn is_a_ins(ins: &str) -> bool {
        if ins.len() > 0 && ins.chars().next().unwrap() == '@' {
            true
        } else {
            false
        }
    }
    fn is_c_ins(ins: &str) -> bool {
        // list all c-instruction possible, but I will sample skip it
        match ins {
            "M=1" => true,
            // "D=1" => true,
            // ...
            // ...
            _ => false,
        }
    }
    fn is_l_ins(ins: &str) -> bool {
        if ins.len() > 0 && ins.chars().next().unwrap() == '(' {
            true
        } else {
            false
        }
    }
    fn get_type(ins: &str) -> Self {
        let mut t = Self::CInstruction;
        if Self::is_a_ins(ins) {
            t = Self::AInstruction;
        }
        if Self::is_l_ins(ins) {
            t = Self::LInstruction;
        }
        t
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    ins_type: InstructionType,
    ins_raw: String,
}

struct Parser {
    lines: Vec<String>,
    next_line_number: usize,
    current_instruction: Option<Instruction>,
}

impl Parser {
    fn new(path: &str) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let lines = contents.lines().map(|s| String::from(s)).collect();

        Ok(Self {
            lines,
            next_line_number: 0,
            current_instruction: None,
        })
    }

    fn has_more_lines(&self) -> bool {
        self.next_line_number < self.lines.len()
    }

    fn advance(&mut self) {
        // 0. if has no more line, return
        // 1. 读一行，去掉注释后内容
        // 2. trim左右
        // 3. 等于""，则跳过改行，取下一行，next line number + 1, 跳回 step 0.
        //    不等于"", 则设为current ins,结束。

        loop {
            if !self.has_more_lines() {
                return;
            }
            let mut line = self.lines[self.next_line_number].clone();
            self.next_line_number += 1;

            if let Some(index) = line.find("//") {
                line = line[..index].to_string()
            }
            let line = line.trim();
            if line != "" {
                let ins_raw = line.to_string();
                let ins_type = InstructionType::get_type(&ins_raw);
                self.current_instruction = Some(Instruction { ins_type, ins_raw });
                return;
            }
        }
    }

    fn symbol(&self) -> String {
        "".to_string()
    }
    fn dest(&self) -> String {
        "".to_string()
    }
    fn comp(&self) -> String {
        "".to_string()
    }
    fn jump(&self) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, OpenOptions},
        io::{Seek, Write},
    };

    use super::*;

    #[test]
    fn test_lines() -> io::Result<()> {
        let test_file = TestFile::new()?;
        let parser = Parser::new(&test_file.path)?;

        assert_eq!(parser.lines.len(), test_file.total_lines);

        Ok(())
    }

    #[test]
    fn test_has_more_lines() -> io::Result<()> {
        let test_file = TestFile::new()?;
        let mut parser = Parser::new(&test_file.path)?;

        assert_eq!(parser.next_line_number, 0);

        parser.next_line_number = test_file.total_lines - 1;
        assert!(parser.has_more_lines());

        parser.next_line_number = test_file.total_lines;
        assert!(!parser.has_more_lines());

        Ok(())
    }

    #[test]
    fn test_empty() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        let parser = Parser::new(&test_file.path)?;

        assert_eq!(parser.current_instruction, None);
        assert_eq!(parser.next_line_number, 0);
        assert!(!parser.has_more_lines());

        Ok(())
    }

    #[test]
    fn test_advance() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("//comment1")?;
        test_file.add_line("")?;
        test_file.add_line("//comment2")?;
        test_file.add_line(" (LOOP) //comment3")?;
        test_file.add_line(" @123 //comment4")?;
        test_file.add_line(" M=1 //comment5")?;
        let mut parser = Parser::new(&test_file.path)?;
        let prev_nln = parser.next_line_number;

        assert_eq!(parser.current_instruction, None);
        assert_eq!(prev_nln, 0);
        assert!(parser.has_more_lines());

        parser.advance();

        assert_eq!(
            parser.current_instruction,
            Some(Instruction {
                ins_raw: String::from("(LOOP)"),
                ins_type: InstructionType::LInstruction
            })
        );
        assert_eq!(parser.next_line_number, prev_nln + 4);
        assert!(parser.has_more_lines());

        parser.advance();

        assert_eq!(
            parser.current_instruction,
            Some(Instruction {
                ins_raw: String::from("@123"),
                ins_type: InstructionType::AInstruction
            })
        );
        assert_eq!(parser.next_line_number, prev_nln + 5);
        assert!(parser.has_more_lines());

        parser.advance();

        assert_eq!(
            parser.current_instruction,
            Some(Instruction {
                ins_raw: String::from("M=1"),
                ins_type: InstructionType::CInstruction
            })
        );
        assert_eq!(parser.next_line_number, prev_nln + 6);
        assert!(!parser.has_more_lines());

        Ok(())
    }

    // todo
    // #[test]
    // fn test_symbol() -> io::Result<()> {
    //     let test_file = TestFile::new()?;
    //     let parser = Parser::new(&test_file.path)?;

    //     assert_eq!();

    //     Ok(())
    // }

    // test template
    // #[test]
    // fn test_xxx() -> io::Result<()> {
    //     let test_file = TestFile::new()?;
    //     let parser = Parser::new(&test_file.path)?;

    //     assert_eq!();

    //     Ok(())
    // }

    struct TestFile {
        file: File,
        path: String,
        total_lines: usize,
    }

    impl TestFile {
        fn new() -> io::Result<Self> {
            let path = "./test_lines.asm";
            let total_lines = 10;
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                // .append(true)
                .create(true)
                .open(path)?;

            for i in 0..total_lines {
                file.write_all(format!("{i}\n").as_bytes())?
            }
            Ok(Self {
                file,
                path: path.to_string(),
                total_lines,
            })
        }

        fn clear(&mut self) -> io::Result<()> {
            self.file.seek(io::SeekFrom::Start(0))?;
            self.file.set_len(0)?;
            self.total_lines = 0;
            Ok(())
        }

        fn add_line(&mut self, newline: &str) -> io::Result<()> {
            self.file.write_all(format!("{newline}\n").as_bytes())?;
            self.total_lines += 1;

            Ok(())
        }
    }

    impl Drop for TestFile {
        fn drop(&mut self) {
            fs::remove_file(&mut self.path)
                .expect(format!("remove test file `{}` fail...", &self.path).as_str());
        }
    }
}
