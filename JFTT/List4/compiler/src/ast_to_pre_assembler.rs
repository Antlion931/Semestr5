use crate::ast::*;
use crate::pre_assembler::*;

pub fn ast_to_pre_assembler(ast: AST) -> Block {
    let mut result = Block::new();

    let mut index = 0;

    for d in ast.main.declarations {
        match d.decl_type {
            DeclType::Number => {
                result
                    .memory
                    .insert(AbstractVarible::Else(AbstractNumber::Var(d.name)), index);
                index += 1;
            }
            DeclType::Table(x) => {
                for i in 0..x {
                    result.memory.insert(
                        AbstractVarible::Table(d.name.clone(), AbstractNumber::Const(i)),
                        index,
                    );
                    index += 1;
                }
            }
        }
    }

    for com in ast.main.commands {
        match com {
            Command::Assign(x) => match x.expression {
                Expression::Number(x) => {
                    unimplemented!();
                }
                Expression::Add(a, b) => {
                    unimplemented!();
                }
                Expression::Sub(a, b) => {
                    unimplemented!();
                }
                Expression::Mul(a, b) => {
                    unimplemented!();
                }
                Expression::Div(a, b) => {
                    unimplemented!();
                }
                Expression::Mod(a, b) => {
                    unimplemented!();
                }
            },
            Command::If(x) => {
                unimplemented!();
            }
            Command::While(x) => {
                unimplemented!();
            }
            Command::Repeat(x) => {
                unimplemented!();
            }
            Command::Call(x) => {
                unimplemented!();
            }
            Command::Read(x) => {
                result.pre_assembler.push(PreAssembler::READ);
                result
                    .pre_assembler
                    .push(PreAssembler::PUT(identifier_to_abstract_varible(
                        x.identifier,
                    )));
            }
            Command::Write(x) => {
                result
                    .pre_assembler
                    .push(PreAssembler::GET(value_to_abstract_varible(x.value)));
                result.pre_assembler.push(PreAssembler::WRITE);
            }
        }
    }

    result
}

fn identifier_to_abstract_varible(id: Identifier) -> AbstractVarible {
    match id.indentifier_type {
        IdentifierType::Number => AbstractVarible::Else(AbstractNumber::Var(id.name)),
        IdentifierType::TableWithNumber(x) => {
            AbstractVarible::Table(id.name, AbstractNumber::Const(x))
        }
        IdentifierType::TableWithIdentifier(x) => {
            AbstractVarible::Table(id.name, AbstractNumber::Var(x))
        }
    }
}

fn value_to_abstract_varible(val: Value) -> AbstractVarible {
    match val {
        Value::Number(x) => AbstractVarible::Else(AbstractNumber::Const(x)),
        Value::Identifier(x) => identifier_to_abstract_varible(x),
    }
}
