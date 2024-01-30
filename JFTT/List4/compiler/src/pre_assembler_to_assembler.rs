use crate::pre_assembler::*;
use std::collections::HashMap;

pub fn pre_assembler_to_assembler(block: Block, compile_info: CompileInfo) -> String {
    let mut result = String::new();

    for pre in block.pre_assembler {
        match pre {
            PreAssembler::READ => {
                result.push_str("# READ\n");
                result.push_str("READ\n");
            }
            PreAssembler::WRITE => {
                result.push_str("# WRITE\n");
                result.push_str("WRITE\n");
            }
            PreAssembler::ADD(ref x) => {
                result.push_str(format!("# ADD {:?}\n", x).as_str());
                match x {
                    AbstractVarible::Table(p, t, i) => {
                        let x = AbstractVarible::Table(p.clone(), t.clone(), AbstractNumber::Const(0));
                        let mem = compile_info.memory.get(&x).unwrap();

                        match i {
                            AbstractNumber::Const(xx) => {
                                let mem = mem + xx;

                                result.push_str("PUT c\n");
                                result.push_str(&generate_const_in_reg('b', mem));
                                result.push_str("LOAD b\n");
                                result.push_str("ADD c\n");
                            }
                            xx @ AbstractNumber::Var(_, _) => {
                                let xx = AbstractVarible::Else(xx.clone());
                                let xx_mem = compile_info.memory.get(&xx).unwrap();

                                result.push_str("PUT c\n");
                                result.push_str(&generate_const_in_reg('b', *xx_mem));
                                result.push_str("LOAD b\n");
                                result.push_str(&generate_const_in_reg('b', *mem));
                                result.push_str("ADD b\n");
                                result.push_str("LOAD a\n");
                                result.push_str("ADD c\n");
                            }
                            xx @ AbstractNumber::Pointer(_) => {
                                let xx = AbstractVarible::Else(xx.clone());
                                let xx_mem = compile_info.memory.get(&xx).unwrap();

                                result.push_str("PUT c\n");
                                result.push_str(&generate_const_in_reg('b', *xx_mem));
                                result.push_str("LOAD b\n");
                                result.push_str("LOAD a\n");

                                result.push_str(&generate_const_in_reg('b', *mem));
                                result.push_str("ADD b\n");
                                result.push_str("LOAD a\n");
                                result.push_str("ADD c\n");
                            }
                            _ => {
                                unimplemented!();
                            }
                        }
                    }
                    AbstractVarible::Else(xx) => match xx {
                        AbstractNumber::Accumulator => {
                            result.push_str("ADD a\n");
                        }
                        AbstractNumber::Const(x) => {
                            result.push_str(&generate_const_in_reg('b', *x));

                            result.push_str("ADD b\n");
                        }
                        AbstractNumber::ProcedureReturn(p) => {
                            unimplemented!();
                        }
                        AbstractNumber::Pointer(p) => {
                            match p.as_ref() {
                                AbstractVarible::Table(p, t, i) => {
                                    let x = AbstractVarible::Else(AbstractNumber::Pointer(Box::new(AbstractVarible::Table(p.clone(), t.clone(), AbstractNumber::Const(0)))));
                                    let mem = compile_info.memory.get(&x).unwrap();

                                    match i {
                                        AbstractNumber::Const(xx) => {
                                            result.push_str("PUT c\n");
                                            result.push_str(&generate_const_in_reg('b', *mem));
                                            result.push_str("LOAD b\n");

                                            result.push_str(&generate_const_in_reg('b', *xx));
                                            result.push_str("ADD b\n");

                                            result.push_str("LOAD a\n");
                                            result.push_str("ADD c\n");
                                        }
                                        xx @ AbstractNumber::Var(_, _) => {
                                            let xx = AbstractVarible::Else(xx.clone());
                                            let xx_mem = compile_info.memory.get(&xx).unwrap();

                                            result.push_str("PUT d\n");
                                            result.push_str(&generate_const_in_reg('b', *xx_mem));
                                            result.push_str("LOAD b\n");
                                            result.push_str("PUT c\n");

                                            result.push_str(&generate_const_in_reg('b', *mem));
                                            result.push_str("LOAD b\n");

                                            result.push_str("ADD c\n");
                                            result.push_str("LOAD a\n");
                                            result.push_str("ADD d\n");
                                        }
                                        xx @ AbstractNumber::Pointer(_) => {
                                            let xx = AbstractVarible::Else(xx.clone());
                                            let xx_mem = compile_info.memory.get(&xx).unwrap();

                                            result.push_str("PUT d\n");
                                            result.push_str(&generate_const_in_reg('b', *xx_mem));
                                            result.push_str("LOAD b\n");
                                            result.push_str("LOAD a\n");
                                            result.push_str("PUT c\n");

                                            result.push_str(&generate_const_in_reg('b', *mem));
                                            result.push_str("LOAD b\n");

                                            result.push_str("ADD c\n");
                                            result.push_str("LOAD a\n");
                                            result.push_str("ADD d\n");
                                        }
                                        _ => {
                                            unimplemented!();
                                        }
                                    }
                                }
                                AbstractVarible::Else(xx) => {
                                    let mem = compile_info.memory.get(x).unwrap();

                                    result.push_str(&generate_const_in_reg('b', *mem));

                                    result.push_str("PUT c\n");

                                    result.push_str("LOAD b\n");
                                    result.push_str("LOAD a\n");

                                    result.push_str("ADD c\n");
                                }
                            }
                        }
                        AbstractNumber::Var(_, ref xxx) => {
                            let mem = compile_info.memory.get(x).unwrap();

                            result.push_str(&generate_const_in_reg('b', *mem));

                            result.push_str("PUT c\n");

                            result.push_str("LOAD b\n");

                            result.push_str("ADD c\n");
                        }
                    },
                }
            }
            PreAssembler::SUB(ref x) => {
                result.push_str(format!("# SUB {:?}\n", x).as_str());
                match x {
                    AbstractVarible::Table(p, t, i) => {
                        let x = AbstractVarible::Table(p.clone(), t.clone(), AbstractNumber::Const(0));
                        let mem = compile_info.memory.get(&x).unwrap();

                        match i {
                            AbstractNumber::Const(xx) => {
                                let mem = mem + xx;

                                result.push_str("PUT c\n");
                                result.push_str(&generate_const_in_reg('b', mem));
                                result.push_str("LOAD b\n");
                                result.push_str("PUT d\n");
                                result.push_str("GET c\n");
                                result.push_str("SUB d\n");
                            }
                            xx @ AbstractNumber::Var(_, _) => {
                                let xx = AbstractVarible::Else(xx.clone());
                                let xx_mem = compile_info.memory.get(&xx).unwrap();

                                result.push_str("PUT c\n");
                                result.push_str(&generate_const_in_reg('b', *xx_mem));
                                result.push_str("LOAD b\n");
                                result.push_str(&generate_const_in_reg('b', *mem));
                                result.push_str("ADD b\n");
                                result.push_str("LOAD a\n");
                                result.push_str("PUT d\n");
                                result.push_str("GET c\n");
                                result.push_str("SUB d\n");
                            }
                            xx @ AbstractNumber::Pointer(_) => {
                                let xx = AbstractVarible::Else(xx.clone());
                                let xx_mem = compile_info.memory.get(&xx).unwrap();

                                result.push_str("PUT c\n");
                                result.push_str(&generate_const_in_reg('b', *xx_mem));
                                result.push_str("LOAD b\n");
                                result.push_str("LOAD a\n");

                                result.push_str(&generate_const_in_reg('b', *mem));
                                result.push_str("ADD b\n");
                                result.push_str("LOAD a\n");

                                result.push_str("PUT d\n");
                                result.push_str("GET c\n");
                                result.push_str("SUB d\n");
                            }
                            _ => {
                                unimplemented!();
                            }
                        }
                    }
                    AbstractVarible::Else(xx) => match xx {
                        AbstractNumber::Accumulator => {
                            result.push_str("SUB a\n");
                        }
                        AbstractNumber::Const(x) => {
                            result.push_str(&generate_const_in_reg('b', *x));

                            result.push_str("SUB b\n");
                        }
                        AbstractNumber::ProcedureReturn(p) => {
                            unimplemented!();
                        }
                        AbstractNumber::Pointer(p) => {
                            match p.as_ref() {
                                AbstractVarible::Table(p, t, i) => {
                                    let x = AbstractVarible::Else(AbstractNumber::Pointer(Box::new(AbstractVarible::Table(p.clone(), t.clone(), AbstractNumber::Const(0)))));
                                    let mem = compile_info.memory.get(&x).unwrap();

                                    match i {
                                        AbstractNumber::Const(xx) => {
                                            result.push_str("PUT c\n");
                                            result.push_str(&generate_const_in_reg('b', *mem));
                                            result.push_str("LOAD b\n");

                                            result.push_str(&generate_const_in_reg('b', *xx));
                                            result.push_str("ADD b\n");

                                            result.push_str("LOAD a\n");
                                            result.push_str("PUT d\n");
                                            result.push_str("GET c\n");
                                            result.push_str("SUB d\n");
                                        }
                                        xx @ AbstractNumber::Var(_, _) => {
                                            let xx = AbstractVarible::Else(xx.clone());
                                            let xx_mem = compile_info.memory.get(&xx).unwrap();

                                            result.push_str("PUT d\n");
                                            result.push_str(&generate_const_in_reg('b', *xx_mem));
                                            result.push_str("LOAD b\n");
                                            result.push_str("PUT c\n");

                                            result.push_str(&generate_const_in_reg('b', *mem));
                                            result.push_str("LOAD b\n");

                                            result.push_str("ADD c\n");
                                            result.push_str("LOAD a\n");
                                            result.push_str("PUT e\n");
                                            result.push_str("GET d\n");
                                            result.push_str("SUB e\n");
                                        }
                                        xx @ AbstractNumber::Pointer(_) => {
                                            let xx = AbstractVarible::Else(xx.clone());
                                            let xx_mem = compile_info.memory.get(&xx).unwrap();

                                            result.push_str("PUT d\n");
                                            result.push_str(&generate_const_in_reg('b', *xx_mem));
                                            result.push_str("LOAD b\n");
                                            result.push_str("LOAD a\n");
                                            result.push_str("PUT c\n");

                                            result.push_str(&generate_const_in_reg('b', *mem));
                                            result.push_str("LOAD b\n");

                                            result.push_str("ADD c\n");
                                            result.push_str("LOAD a\n");
                                            result.push_str("PUT e\n");
                                            result.push_str("GET d\n");
                                            result.push_str("SUB e\n");
                                        }
                                        _ => {
                                            unimplemented!();
                                        }
                                    }
                                }
                                AbstractVarible::Else(xx) => {
                                    let mem = compile_info.memory.get(x).unwrap();

                                    result.push_str(&generate_const_in_reg('b', *mem));

                                    result.push_str("PUT c\n");

                                    result.push_str("LOAD b\n");
                                    result.push_str("LOAD a\n");

                                    result.push_str("PUT d\n");

                                    result.push_str("GET c\n");

                                    result.push_str("SUB d\n");
                                }
                            }
                        }
                        AbstractNumber::Var(_, ref xxx) => {
                            let mem = compile_info.memory.get(x).unwrap();

                            result.push_str(&generate_const_in_reg('b', *mem));

                            result.push_str("PUT c\n");

                            result.push_str("LOAD b\n");

                            result.push_str("PUT d\n");

                            result.push_str("GET c\n");

                            result.push_str("SUB d\n");
                        }
                    },
                }
            }
            PreAssembler::GET(ref x) => {
                result.push_str(format!("# GET {:?}\n", x).as_str());
                match x {
                    AbstractVarible::Table(p, t, i) => {
                        let x = AbstractVarible::Table(p.clone(), t.clone(), AbstractNumber::Const(0));
                        let mem = compile_info.memory.get(&x).unwrap();

                        match i {
                            AbstractNumber::Const(xx) => {
                                let mem = mem + xx;

                                result.push_str(&generate_const_in_reg('b', mem));
                                result.push_str("LOAD b\n");
                            }
                            xx @ AbstractNumber::Var(_, _) => {
                                let xx = AbstractVarible::Else(xx.clone());
                                let xx_mem = compile_info.memory.get(&xx).unwrap();

                                result.push_str(&generate_const_in_reg('b', *xx_mem));
                                result.push_str("LOAD b\n");
                                result.push_str(&generate_const_in_reg('b', *mem));
                                result.push_str("ADD b\n");
                                result.push_str("LOAD a\n");
                            }
                            xx @ AbstractNumber::Pointer(_) => {
                                let xx = AbstractVarible::Else(xx.clone());
                                let xx_mem = compile_info.memory.get(&xx).unwrap();

                                result.push_str(&generate_const_in_reg('b', *xx_mem));
                                result.push_str("LOAD b\n");
                                result.push_str("LOAD a\n");

                                result.push_str(&generate_const_in_reg('b', *mem));
                                result.push_str("ADD b\n");
                                result.push_str("LOAD a\n");
                            }
                            _ => {
                                unimplemented!();
                            }
                        }
                    }
                    AbstractVarible::Else(xx) => match xx {
                        AbstractNumber::Accumulator => {
                            panic!("Trying to put accumulator to accumulator");
                        }
                        AbstractNumber::Const(x) => {
                            result.push_str(&generate_const_in_reg('a', *x));
                        }
                        AbstractNumber::ProcedureReturn(p) => {
                            let mem = compile_info.memory.get(x).unwrap();

                            result.push_str(&generate_const_in_reg('b', *mem));

                            result.push_str("LOAD b\n");
                        }
                        AbstractNumber::Pointer(p) => {
                            match p.as_ref() {
                                AbstractVarible::Table(p, t, i) => {
                                    let x = AbstractVarible::Else(AbstractNumber::Pointer(Box::new(AbstractVarible::Table(p.clone(), t.clone(), AbstractNumber::Const(0)))));
                                    let mem = compile_info.memory.get(&x).unwrap();

                                    match i {
                                        AbstractNumber::Const(xx) => {
                                            result.push_str(&generate_const_in_reg('b', *mem));
                                            result.push_str("LOAD b\n");

                                            result.push_str(&generate_const_in_reg('b', *xx));
                                            result.push_str("ADD b\n");

                                            result.push_str("LOAD a\n");
                                        }
                                        xx @ AbstractNumber::Var(_, _) => {
                                            let xx = AbstractVarible::Else(xx.clone());
                                            let xx_mem = compile_info.memory.get(&xx).unwrap();

                                            result.push_str(&generate_const_in_reg('b', *xx_mem));
                                            result.push_str("LOAD b\n");
                                            result.push_str("PUT c\n");

                                            result.push_str(&generate_const_in_reg('b', *mem));
                                            result.push_str("LOAD b\n");

                                            result.push_str("ADD c\n");
                                            result.push_str("LOAD a\n");
                                        }
                                        xx @ AbstractNumber::Pointer(_) => {
                                            let xx = AbstractVarible::Else(xx.clone());
                                            let xx_mem = compile_info.memory.get(&xx).unwrap();

                                            result.push_str(&generate_const_in_reg('b', *xx_mem));
                                            result.push_str("LOAD b\n");
                                            result.push_str("LOAD a\n");
                                            result.push_str("PUT c\n");

                                            result.push_str(&generate_const_in_reg('b', *mem));
                                            result.push_str("LOAD b\n");

                                            result.push_str("ADD c\n");
                                            result.push_str("LOAD a\n");

                                        }
                                        _ => {
                                            unimplemented!();
                                        }
                                    }
                                }
                                AbstractVarible::Else(xx) => {
                                    let mem = compile_info.memory.get(x).unwrap();

                                    result.push_str(&generate_const_in_reg('b', *mem));

                                    result.push_str("LOAD b\n");
                                    result.push_str("LOAD a\n");
                                }
                            }
                        }
                        AbstractNumber::Var(_, n) => {
                            let mem = compile_info.memory.get(x).unwrap();

                            result.push_str(&generate_const_in_reg('b', *mem));

                            result.push_str("LOAD b\n");
                        }
                    },
                }
            }
            PreAssembler::PUT(ref x) => {
                result.push_str(format!("# PUT {:?}\n", x).as_str());
                match x {
                    AbstractVarible::Table(p, t, i) => {
                        let x = AbstractVarible::Table(p.clone(), t.clone(), AbstractNumber::Const(0));
                        let mem = compile_info.memory.get(&x).unwrap();

                        match i {
                            AbstractNumber::Const(xx) => {
                                let mem = mem + xx;

                                result.push_str(&generate_const_in_reg('b', mem));

                                result.push_str("STORE b\n");
                            }
                            xx @ AbstractNumber::Var(_, _) => {
                                let xx = AbstractVarible::Else(xx.clone());
                                let xx_mem = compile_info.memory.get(&xx).unwrap();
                                result.push_str("PUT c\n");
                                result.push_str(&generate_const_in_reg('b', *xx_mem));
                                result.push_str("LOAD b\n");
                                result.push_str(&generate_const_in_reg('b', *mem));
                                result.push_str("ADD b\n");
                                result.push_str("PUT d\n");
                                result.push_str("GET c\n");
                                result.push_str("STORE d\n");
                            }
                            xx @ AbstractNumber::Pointer(_) => {
                                let xx = AbstractVarible::Else(xx.clone());
                                let xx_mem = compile_info.memory.get(&xx).unwrap();

                                result.push_str("PUT c\n");
                                result.push_str(&generate_const_in_reg('b', *xx_mem));
                                result.push_str("LOAD b\n");
                                result.push_str("LOAD a\n");

                                result.push_str(&generate_const_in_reg('b', *mem));
                                result.push_str("ADD b\n");
                                result.push_str("PUT d\n");
                                result.push_str("GET c\n");
                                result.push_str("STORE d\n");
                            }
                            _ => {
                                unimplemented!();
                            }
                        }
                    }
                    AbstractVarible::Else(xx) => match xx {
                        AbstractNumber::Accumulator => {
                            panic!("Trying to put accumulator to accumulator");
                        }
                        AbstractNumber::Const(x) => {
                            panic!("Trying to put accumulator to const");
                        }
                        AbstractNumber::ProcedureReturn(p) => {
                            unimplemented!();
                        }
                        AbstractNumber::Pointer(p) => {
                            match p.as_ref() {
                                AbstractVarible::Table(p, t, i) => {
                                    let x = AbstractVarible::Else(AbstractNumber::Pointer(Box::new(AbstractVarible::Table(p.clone(), t.clone(), AbstractNumber::Const(0))))); 
                                    let mem = compile_info.memory.get(&x).unwrap();

                                    match i {
                                        AbstractNumber::Const(xx) => {
                                            result.push_str("PUT c\n");
                                            result.push_str(&generate_const_in_reg('b', *mem));
                                            result.push_str("LOAD b\n");

                                            result.push_str(&generate_const_in_reg('b', *xx));
                                            result.push_str("ADD b\n");

                                            result.push_str("PUT d\n");
                                            result.push_str("GET c\n");

                                            result.push_str("STORE d\n");
                                        }
                                        xx @ AbstractNumber::Var(_, _) => {
                                            let xx = AbstractVarible::Else(xx.clone());
                                            let xx_mem = compile_info.memory.get(&xx).unwrap();
                                            result.push_str("PUT c\n");
                                            result.push_str(&generate_const_in_reg('b', *xx_mem));
                                            result.push_str("LOAD b\n");
                                            result.push_str("PUT d\n");
                                            result.push_str(&generate_const_in_reg('b', *mem));
                                            result.push_str("LOAD b\n");
                                            result.push_str("ADD d\n");
                                            result.push_str("PUT d\n");
                                            result.push_str("GET c\n");
                                            result.push_str("STORE d\n");
                                        }
                                        xx @ AbstractNumber::Pointer(_) => {
                                            let xx = AbstractVarible::Else(xx.clone());
                                            let xx_mem = compile_info.memory.get(&xx).unwrap();
                                            result.push_str("PUT c\n");
                                            result.push_str(&generate_const_in_reg('b', *xx_mem));
                                            result.push_str("LOAD b\n");
                                            result.push_str("LOAD a\n");
                                            result.push_str("PUT d\n");
                                            result.push_str(&generate_const_in_reg('b', *mem));
                                            result.push_str("LOAD b\n");
                                            result.push_str("ADD d\n");
                                            result.push_str("PUT d\n");
                                            result.push_str("GET c\n");
                                            result.push_str("STORE d\n");
                                        }
                                        _ => {
                                            unimplemented!();
                                        }
                                    }
                                }
                                AbstractVarible::Else(xx) => match xx {
                                    AbstractNumber::Accumulator => {
                                        panic!("Trying to put accumulator to accumulator");
                                    }
                                    AbstractNumber::Const(x) => {
                                        panic!("Trying to put accumulator to const");
                                    }
                                    AbstractNumber::ProcedureReturn(p) => {
                                        unimplemented!();
                                    }
                                    AbstractNumber::Pointer(p) => {
                                        unimplemented!();
                                    }
                                    AbstractNumber::Var(_, n) => {
                                        let mem = compile_info.memory.get(x).unwrap(); //Error here

                                        result.push_str(&generate_const_in_reg('b', *mem));
                                        result.push_str("PUT c\n");
                                        result.push_str("LOAD b\n");
                                        result.push_str("PUT d\n");
                                        result.push_str("GET c\n");

                                        result.push_str("STORE d\n");

                                    }
                                },
                            }
                                
                        }
                        AbstractNumber::Var(_, n) => {
                            let mem = compile_info.memory.get(x).unwrap();

                            result.push_str(&generate_const_in_reg('b', *mem));

                            result.push_str("STORE b\n");
                        }
                    },
                }
            }
            PreAssembler::INC(x) => {
                result.push_str(format!("# INC {:?}\n", x).as_str());
                match x {
                    AbstractVarible::Table(_, t, i) => {
                        unimplemented!();
                    }
                    AbstractVarible::Else(xx) => match xx {
                        AbstractNumber::Accumulator => {
                            result.push_str("INC a\n");
                        }
                        AbstractNumber::Const(x) => {
                            unimplemented!();
                        }
                        AbstractNumber::ProcedureReturn(p) => {
                            unimplemented!();
                        }
                        AbstractNumber::Pointer(p) => {
                            unimplemented!();
                        }
                        AbstractNumber::Var(_, n) => {
                            unimplemented!();
                        }
                    },
                }
            }
            PreAssembler::LABEL(x) => {
                result.push_str(format!("# LABEL {}\n", x).as_str());
            }
            PreAssembler::JUMP(x) => {
                result.push_str(format!("# JUMP {}\n", x).as_str());
                result.push_str(format!("JUMP {}\n", x).as_str());
            }
            PreAssembler::JPOS(x) => {
                result.push_str(format!("# JPOS {}\n", x).as_str());
                result.push_str(format!("JPOS {}\n", x).as_str());
            }
            PreAssembler::JZERO(x) => {
                result.push_str(format!("# JZERO {}\n", x).as_str());
                result.push_str(format!("JZERO {}\n", x).as_str());
            }
            PreAssembler::JUMPR(x) => {
                result.push_str("# JUMPR\n");
                match x {
                    AbstractVarible::Else(AbstractNumber::Accumulator) => {
                        result.push_str("JUMPR a\n");
                    }
                    _ => {
                        panic!("JUMPR can only be used with accumulator");
                    }
                }
            }
            PreAssembler::STRK(ref x) => {
                result.push_str("# STRK\n");
                match x {
                    AbstractVarible::Else(AbstractNumber::ProcedureReturn(_)) => {
                        let mem = compile_info.memory.get(x).unwrap();
                        result.push_str(&generate_const_in_reg('b', *mem));
                        result.push_str("STRK a\n");
                        result.push_str("STORE b\n");
                    }
                    _ => {
                        panic!("STRK can only be used with procedure return");
                    }
                }
            }
            PreAssembler::HALT => {
                result.push_str("# HALT\n");
                result.push_str("HALT\n");
            }
            PreAssembler::MUL => {
                result.push_str("# MUL\n");
                result.push_str("RST d\n");
                result.push_str("STRK e\n");
                result.push_str(format!("JUMP {}\n", compile_info.mul_label.unwrap()).as_str());
            }
            PreAssembler::DIV => {
                result.push_str("# DIV\n");
                result.push_str("STRK e\n");
                result.push_str(format!("JUMP {}\n", compile_info.div_label.unwrap()).as_str());
            }
            PreAssembler::MOD => {
                result.push_str("# MOD\n");
                result.push_str("RST d\n");
                result.push_str("STRK e\n");
                result.push_str(format!("JUMP {}\n", compile_info.mod_label.unwrap()).as_str());
            }
            PreAssembler::MOVE(x) => {
                result.push_str(format!("# MOVE {}\n", x).as_str());
                result.push_str(format!("PUT {}\n", x).as_str());
            }
        }
    }

    if let Some(x) = compile_info.mul_label {
        result.push_str(format!("# LABEL {}\n", x).as_str());
        result.push_str("GET g\n");
        result.push_str(format!("JZERO {}\n", x + 1).as_str());
        result.push_str("SHR g\n");
        result.push_str("SUB g\n");
        result.push_str("SUB g\n");
        result.push_str(format!("JZERO {}\n", x + 2).as_str());
        result.push_str("GET d\n");
        result.push_str("ADD h\n");
        result.push_str("PUT d\n");
        result.push_str(format!("# LABEL {}\n", x + 2).as_str());
        result.push_str("SHL h\n");
        result.push_str(format!("JUMP {}\n", x).as_str());
        result.push_str(format!("# LABEL {}\n", x + 1).as_str());
        result.push_str("GET d\n");
        result.push_str("INC e\n");
        result.push_str("INC e\n");
        result.push_str("JUMPR e\n");
    }

    if let Some(x) = compile_info.div_label {
        result.push_str(format!("# LABEL {}\n", x).as_str());
        result.push_str("RST d\n");
        result.push_str("GET g\n");
        result.push_str(format!("JZERO {}\n", x + 1).as_str());
        result.push_str("PUT c\n");
        result.push_str(format!("# LABEL {}\n", x + 2).as_str());
        result.push_str("GET h\n");
        result.push_str("SUB c\n");
        result.push_str(format!("JZERO {}\n", x + 3).as_str());
        result.push_str("SHL c\n");
        result.push_str(format!("JUMP {}\n", x + 2).as_str());
        result.push_str(format!("# LABEL {}\n", x + 3).as_str());
        result.push_str("GET g\n");
        result.push_str("SUB c\n");
        result.push_str(format!("JPOS {}\n", x + 1).as_str());
        result.push_str("SHL d\n");
        result.push_str("GET c\n");
        result.push_str("SUB h\n");
        result.push_str(format!("JPOS {}\n", x + 4).as_str());
        result.push_str("INC d\n");
        result.push_str("GET h\n");
        result.push_str("SUB c\n");
        result.push_str("PUT h\n");
        result.push_str(format!("# LABEL {}\n", x + 4).as_str());
        result.push_str("SHR c\n");
        result.push_str(format!("JUMP {}\n", x + 3).as_str());
        result.push_str(format!("# LABEL {}\n", x + 1).as_str());
        result.push_str("GET d\n");
        result.push_str("INC e\n");
        result.push_str("INC e\n");
        result.push_str("JUMPR e\n");
    }

    if let Some(x) = compile_info.mod_label {
        result.push_str(format!("# LABEL {}\n", x).as_str());
        result.push_str("GET g\n");
        result.push_str(format!("JZERO {}\n", x + 5).as_str());
        result.push_str("PUT c\n");
        result.push_str(format!("# LABEL {}\n", x + 2).as_str());
        result.push_str("GET h\n");
        result.push_str("SUB c\n");
        result.push_str(format!("JZERO {}\n", x + 3).as_str());
        result.push_str("SHL c\n");
        result.push_str(format!("JUMP {}\n", x + 2).as_str());
        result.push_str(format!("# LABEL {}\n", x + 3).as_str());
        result.push_str("GET g\n");
        result.push_str("SUB c\n");
        result.push_str(format!("JPOS {}\n", x + 1).as_str());
        result.push_str("GET c\n");
        result.push_str("SUB h\n");
        result.push_str(format!("JPOS {}\n", x + 4).as_str());
        result.push_str("GET h\n");
        result.push_str("SUB c\n");
        result.push_str("PUT h\n");
        result.push_str(format!("# LABEL {}\n", x + 4).as_str());
        result.push_str("SHR c\n");
        result.push_str(format!("JUMP {}\n", x + 3).as_str());
        result.push_str(format!("# LABEL {}\n", x + 5).as_str());
        result.push_str("RST h\n");
        result.push_str(format!("# LABEL {}\n", x + 1).as_str());
        result.push_str("GET h\n");
        result.push_str("INC e\n");
        result.push_str("INC e\n");
        result.push_str("JUMPR e\n");
    }

    // Change Labels to numbers
    let mut k = 0;

    let mut labels_to_lines = HashMap::new();

    for line in result.lines() {
        if line.starts_with("# LABEL ") {
            let label = line.split(" ").collect::<Vec<&str>>()[2];
            let label = label.parse::<usize>().unwrap();
            labels_to_lines.insert(label, k);
        } else if !line.starts_with("#") {
            k += 1;
        }
    }

    let mut new_result = String::new();

    for line in result.lines() {
        if line.starts_with("JUMP ") {
            let label = line.split(" ").collect::<Vec<&str>>()[1];
            let label = label.parse::<usize>().unwrap();
            let line = labels_to_lines.get(&label).unwrap();
            new_result.push_str(format!("JUMP {}\n", line).as_str());
        } else if line.starts_with("JPOS ") {
            let label = line.split(" ").collect::<Vec<&str>>()[1];
            let label = label.parse::<usize>().unwrap();
            let line = labels_to_lines.get(&label).unwrap();
            new_result.push_str(format!("JPOS {}\n", line).as_str());
        } else if line.starts_with("JZERO ") {
            let label = line.split(" ").collect::<Vec<&str>>()[1];
            let label = label.parse::<usize>().unwrap();
            let line = labels_to_lines.get(&label).unwrap();
            new_result.push_str(format!("JZERO {}\n", line).as_str());
        }else {
            new_result.push_str(line);
            new_result.push_str("\n");
        }
    }

    new_result
}

fn generate_const_in_reg(reg: char, value: u64) -> String {
    let mut result = String::new();

    result.push_str(&format!("RST {}\n", reg));

    let mut first_one = false;

    for i in (0..64).rev() {
        if value & (1 << i) != 0 {
            if first_one {
                result.push_str(&format!("SHL {}\n", reg));
            }
            result.push_str(&format!("INC {}\n", reg));
            first_one = true;
        } else if first_one {
            result.push_str(&format!("SHL {}\n", reg));
        }
    }

    result
}
