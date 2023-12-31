use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum InstructionType {
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
    #[allow(unused)]
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

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Instruction {
    pub ins_type: InstructionType,
    ins_raw: String,
}

pub struct Parser {
    lines: Vec<String>,
    next_line_number: usize,
    pub next_ins_address: usize,
    pub current_instruction: Option<Instruction>,
}

impl Parser {
    pub fn new(path: &Path) -> io::Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let lines = contents.lines().map(|s| String::from(s)).collect();

        Ok(Self {
            lines,
            next_line_number: 0,
            next_ins_address: 0,
            current_instruction: None,
        })
    }

    pub fn has_more_lines(&self) -> bool {
        self.next_line_number < self.lines.len()
    }

    /// 0. if has no more line, return
    /// 1. 读一行，去掉注释后内容
    /// 2. trim左右
    /// 3. 等于""，则跳过改行，取下一行，next line number + 1, 跳回 step 0.
    ///    不等于"", 则设为current ins,结束。
    pub fn advance(&mut self) {
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
                if ins_type != InstructionType::LInstruction {
                    self.next_ins_address += 1;
                }
                self.current_instruction = Some(Instruction { ins_type, ins_raw });
                return;
            }
        }
    }

    pub fn symbol(&self) -> String {
        assert!(
            self.current_instruction.is_some(),
            "Can't call symbol() when have not instruction"
        );
        let ins = self.current_instruction.clone().unwrap();
        use InstructionType::*;
        match ins.ins_type {
            AInstruction => ins.ins_raw[1..].to_string(),
            LInstruction => ins.ins_raw[1..(ins.ins_raw.len() - 1)].to_string(),
            CInstruction => panic!("Can't call symbol() in a C-Instruction"),
        }
    }

    pub fn dest(&self) -> String {
        assert!(
            self.current_instruction.is_some(),
            "Can't call dest() when have not instruction"
        );
        let ins = self.current_instruction.clone().unwrap();
        use InstructionType::*;
        match ins.ins_type {
            AInstruction => panic!("Can't call dest() in a A-Instruction"),
            LInstruction => panic!("Can't call dest() in a L-Instruction"),
            CInstruction => {
                let mut dest = "";
                let splited = ins.ins_raw.split("=").collect::<Vec<_>>();
                if splited.len() == 2 {
                    dest = splited[0];
                }
                dest.to_string()
            }
        }
    }

    pub fn comp(&self) -> String {
        assert!(
            self.current_instruction.is_some(),
            "Can't call comp() when have not instruction"
        );
        let ins = self.current_instruction.clone().unwrap();
        use InstructionType::*;
        match ins.ins_type {
            AInstruction => panic!("Can't call comp() in a A-Instruction"),
            LInstruction => panic!("Can't call comp() in a L-Instruction"),
            CInstruction => {
                let dest;
                let splited = ins.ins_raw.split("=").collect::<Vec<_>>();
                let comp_and_jump;
                if splited.len() == 2 {
                    comp_and_jump = splited[1];
                } else {
                    comp_and_jump = splited[0];
                }
                let splited = comp_and_jump.split(";").collect::<Vec<_>>();
                dest = splited[0];

                dest.to_string()
            }
        }
    }

    pub fn jump(&self) -> String {
        assert!(
            self.current_instruction.is_some(),
            "Can't call jump() when have not instruction"
        );
        let ins = self.current_instruction.clone().unwrap();
        use InstructionType::*;
        match ins.ins_type {
            AInstruction => panic!("Can't call jump() in a A-Instruction"),
            LInstruction => panic!("Can't call jump() in a L-Instruction"),
            CInstruction => {
                let mut dest = "";
                let splited = ins.ins_raw.split(";").collect::<Vec<_>>();
                if splited.len() == 2 {
                    dest = splited[1];
                }

                dest.to_string()
            }
        }
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
        let parser = Parser::new(&Path::new(&test_file.path))?;

        assert_eq!(parser.lines.len(), test_file.total_lines);

        Ok(())
    }

    #[test]
    fn test_has_more_lines() -> io::Result<()> {
        let test_file = TestFile::new()?;
        let mut parser = Parser::new(&Path::new(&test_file.path))?;

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
        let parser = Parser::new(&Path::new(&test_file.path))?;

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
        test_file.add_line("//comment6")?;
        let mut parser = Parser::new(&Path::new(&test_file.path))?;
        let prev_nln = parser.next_line_number;

        assert_eq!(parser.current_instruction, None);
        assert_eq!(prev_nln, 0);
        assert_eq!(parser.next_ins_address, 0);
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
        assert_eq!(parser.next_ins_address, 0);
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
        assert_eq!(parser.next_ins_address, 1);
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
        assert_eq!(parser.next_ins_address, 2);
        assert!(parser.has_more_lines());

        parser.advance();

        assert_eq!(
            parser.current_instruction,
            Some(Instruction {
                ins_raw: String::from("M=1"),
                ins_type: InstructionType::CInstruction
            })
        );
        assert_eq!(parser.next_line_number, prev_nln + 7);
        assert!(!parser.has_more_lines());

        Ok(())
    }

    #[test]
    fn test_jump() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("1")?;
        test_file.add_line("M=1")?;
        test_file.add_line("M=1;JMP")?;
        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        parser.advance();
        assert_eq!(parser.jump(), "".to_string());
        parser.advance();
        assert_eq!(parser.jump(), "".to_string());
        parser.advance();
        assert_eq!(parser.jump(), "JMP".to_string());

        Ok(())
    }

    #[test]
    #[should_panic = "Can't call jump() in a A-Instruction"]
    fn test_jump_panic_in_a_ins() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("@123").unwrap();
        let mut parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        parser.advance();
        // should panic
        parser.jump();
    }

    #[test]
    #[should_panic = "Can't call jump() in a L-Instruction"]
    fn test_jump_panic_in_l_ins() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("(LOOP)").unwrap();
        let mut parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        parser.advance();
        // should panic
        parser.jump();
    }

    #[test]
    #[should_panic = "Can't call jump() when have not instruction"]
    fn test_jump_panic_in_no_ins() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("//comment").unwrap();
        let parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        // should panic
        parser.jump();
    }

    #[test]
    fn test_comp() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("1")?;
        test_file.add_line("M=1")?;
        test_file.add_line("M=1;JMP")?;
        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        parser.advance();
        assert_eq!(parser.comp(), "1".to_string());
        parser.advance();
        assert_eq!(parser.comp(), "1".to_string());
        parser.advance();
        assert_eq!(parser.comp(), "1".to_string());

        Ok(())
    }

    #[test]
    #[should_panic = "Can't call comp() in a A-Instruction"]
    fn test_comp_panic_in_a_ins() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("@123").unwrap();
        let mut parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        parser.advance();
        // should panic
        parser.comp();
    }

    #[test]
    #[should_panic = "Can't call comp() in a L-Instruction"]
    fn test_comp_panic_in_l_ins() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("(LOOP)").unwrap();
        let mut parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        parser.advance();
        // should panic
        parser.comp();
    }

    #[test]
    #[should_panic = "Can't call comp() when have not instruction"]
    fn test_comp_panic_in_no_ins() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("//comment").unwrap();
        let parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        // should panic
        parser.comp();
    }

    #[test]
    fn test_dest() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("1")?;
        test_file.add_line("M=1")?;
        test_file.add_line("D=1")?;
        test_file.add_line("DM=1")?;
        test_file.add_line("A=1")?;
        test_file.add_line("AM=1")?;
        test_file.add_line("AD=1")?;
        test_file.add_line("ADM=1")?;
        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        parser.advance();
        assert_eq!(parser.dest(), "".to_string());

        parser.advance();
        assert_eq!(parser.dest(), "M".to_string());

        parser.advance();
        assert_eq!(parser.dest(), "D".to_string());

        parser.advance();
        assert_eq!(parser.dest(), "DM".to_string());

        parser.advance();
        assert_eq!(parser.dest(), "A".to_string());

        parser.advance();
        assert_eq!(parser.dest(), "AM".to_string());

        parser.advance();
        assert_eq!(parser.dest(), "AD".to_string());

        parser.advance();
        assert_eq!(parser.dest(), "ADM".to_string());
        Ok(())
    }

    #[test]
    #[should_panic = "Can't call dest() in a A-Instruction"]
    fn test_dest_panic_in_a_ins() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("@123").unwrap();
        let mut parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        parser.advance();
        // should panic
        parser.dest();
    }

    #[test]
    #[should_panic = "Can't call dest() in a L-Instruction"]
    fn test_dest_panic_in_l_ins() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("(LOOP)").unwrap();
        let mut parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        parser.advance();
        // should panic
        parser.dest();
    }

    #[test]
    #[should_panic = "Can't call dest() when have not instruction"]
    fn test_dest_panic_in_no_ins() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("//comment").unwrap();
        let parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        // should panic
        parser.dest();
    }

    #[test]
    fn test_symbol() -> io::Result<()> {
        let mut test_file = TestFile::new()?;
        test_file.clear()?;
        test_file.add_line("(LOOP)")?;
        test_file.add_line("@123")?;
        test_file.add_line("@num")?;
        let mut parser = Parser::new(&Path::new(&test_file.path))?;

        parser.advance();
        assert_eq!(parser.symbol(), "LOOP".to_string());

        parser.advance();
        assert_eq!(parser.symbol(), "123".to_string());

        parser.advance();
        assert_eq!(parser.symbol(), "num".to_string());

        Ok(())
    }

    #[test]
    #[should_panic = "Can't call symbol() in a C-Instruction"]
    fn test_symbol_panic_in_c_ins() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("M=1").unwrap();
        let mut parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        parser.advance();
        // should panic
        parser.symbol();
    }

    #[test]
    #[should_panic = "Can't call symbol() when have not instruction"]
    fn test_symbol_panic_in_no_ins() {
        let mut test_file = TestFile::new().unwrap();
        test_file.clear().unwrap();
        test_file.add_line("//comment").unwrap();
        let parser = Parser::new(&Path::new(&test_file.path)).unwrap();

        // should panic
        parser.symbol();
    }

    // test template
    // #[test]
    // fn test_xxx() -> io::Result<()> {
    //     let test_file = TestFile::new()?;
    //     let parser = Parser::new(&Path::new(&test_file.path))?;

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
