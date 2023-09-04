mod code;
mod parser;

use code::Code;
use parser::*;
use std::{env::args, error::Error, ffi::OsString, fs::OpenOptions, io::Write, path::Path, result};

fn main() -> result::Result<(), Box<dyn Error>> {
    assert_eq!(args().len(), 2, "assembler need a input file arg");
    let input_file_arg = args().nth(1).unwrap();
    let input_file_path = Path::new(&input_file_arg);
    assert!(input_file_path.is_file());
    let input_file_name = input_file_path.file_name().unwrap();
    let input_file_dir = input_file_path.parent().unwrap();

    let input_file_name_str = input_file_name.to_str().unwrap();
    let output_file_name = OsString::from(input_file_name_str.replace(".asm", ".hack"));
    let output_file_path = input_file_dir.join(output_file_name);

    let mut parser = Parser::new(input_file_path)?;
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_file_path)?;

    loop {
        if !parser.has_more_lines() {
            break;
        }
        parser.advance();
        if let Some(ins) = &parser.current_instruction {
            use InstructionType::*;
            match ins.ins_type {
                AInstruction => {
                    let symbol = parser.symbol();
                    let symbol_bin = format!("{:016b}", symbol.parse::<isize>()?);
                    let bin = symbol_bin + "\n";
                    output_file.write_all(bin.as_bytes())?;
                }
                CInstruction => {
                    let prefix_bin = "111".to_string();
                    let comp_bin = Code::comp(&parser.comp());
                    let dest_bin = Code::dest(&parser.dest());
                    let jump_bin = Code::jump(&parser.jump());
                    let bin = prefix_bin + &comp_bin + &dest_bin + &jump_bin + "\n";
                    output_file.write_all(bin.as_bytes())?;
                }
                LInstruction => (),
            }
        }
    }

    Ok(())
}
