mod code;
mod parser;
mod symbol_table;

use code::Code;
use parser::*;
use std::{env::args, error::Error, ffi::OsString, fs::OpenOptions, io::Write, path::Path, result};
use symbol_table::SymbolTable;

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

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(output_file_path)?;

    let mut symbol_table = SymbolTable::new();

    // first pass
    let mut parser = Parser::new(input_file_path)?;
    loop {
        if !parser.has_more_lines() {
            break;
        }
        parser.advance();
        if let Some(ins) = &parser.current_instruction {
            use InstructionType::*;
            match ins.ins_type {
                // add to symbol table
                LInstruction => {
                    let symbol = parser.symbol();
                    if !symbol_table.contains(&symbol) {
                        symbol_table.add_entry(&symbol, parser.next_ins_address);
                    }
                }
                _ => (),
            }
        }
    }

    // second pass
    let mut parser = Parser::new(input_file_path)?;
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
                    // if symbol is number, then parse it
                    // else check if the symbol in symbol table
                    // if in, then translate to it's value
                    // if not in, then add (symbol, alloc_pos) to table, alloc_pos++
                    let symbol_bin = if let Ok(addr) = symbol.parse::<usize>() {
                        format!("{:016b}", addr)
                    } else {
                        if symbol_table.contains(&symbol) {
                            format!("{:016b}", symbol_table.get_address(&symbol))
                        } else {
                            let alloc_pos = symbol_table.alloc_pos;
                            symbol_table.alloc_pos += 1;
                            symbol_table.add_entry(&symbol, alloc_pos);
                            format!("{:016b}", alloc_pos)
                        }
                    };
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
                _ => (),
            }
        }
    }

    Ok(())
}
