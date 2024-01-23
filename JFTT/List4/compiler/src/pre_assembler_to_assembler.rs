use crate::pre_assembler::*;
use std::collections::HashMap;

pub fn pre_assembler_to_assembler(block: Block) -> String {
    let mut result = String::new();

    for pre in block.pre_assembler {
        match pre {
            PreAssembler::READ => {
                result.push_str("READ\n");
            }
            PreAssembler::WRITE => {
                result.push_str("WRITE\n");
            }
            PreAssembler::ADD(x) => {
                unimplemented!();
            }
            PreAssembler::SUB(x) => {
                unimplemented!();
            }
            PreAssembler::GET(x) => {
                let mem = block.memory.get(&x).unwrap();

                result.push_str(&generate_const_in_reg('a', *mem));

                result.push_str("LOAD a\n");
            }
            PreAssembler::PUT(x) => {
                let mem = block.memory.get(&x).unwrap();

                result.push_str(&generate_const_in_reg('b', *mem));

                result.push_str("STORE b\n");
            }
            PreAssembler::RST(x) => {
                unimplemented!();
            }
            PreAssembler::INC(x) => {
                unimplemented!();
            }
            PreAssembler::DEC(x) => {
                unimplemented!();
            }
            PreAssembler::SHL(x) => {
                unimplemented!();
            }
            PreAssembler::SHR(x) => {
                unimplemented!();
            }
        }
    }

    match block.jumps {
        Jumps::HALT => {
            result.push_str("HALT\n");
        }
        _ => {
            unimplemented!();
        }
    }

    result
}

fn generate_const_in_reg(reg: char, value: u64) -> String {
    let mut result = String::new();

    result.push_str(&format!("RST {}\n", reg));

    let mut first_one = false;

    for i in (0..64).rev() {
        if value & (1 << i) != 0 {
            result.push_str(&format!("INC {}\n", reg));
            result.push_str(&format!("SHL {}\n", reg));
            first_one = true;
        } else if first_one {
            result.push_str(&format!("SHL {}\n", reg));
        }
    }

    result
}
