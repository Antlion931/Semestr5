use crate::ast::*;
use crate::common::*;
use crate::pre_assembler::*;
use std::collections::HashMap;

pub fn ast_to_pre_assembler(ast: AST) -> (Block, CompileInfo) {
    let mut result = Block::new();
    let mut compile_info = CompileInfo::new();

    let mut label_counter = Counter::new();
    let mut index_counter = Counter::new();

    let mut procedures_labels = HashMap::new();

    for p in &ast.procedurs {
        procedures_labels.insert(p.get_name().to_string(), label_counter.next());
    }

    add_commands(
        &ast.main.commands,
        ProcType::Main,
        &mut result,
        &mut compile_info,
        &mut index_counter,
        &mut label_counter,
        &procedures_labels,
        &ast.procedurs,
        &ast.main.declarations,
        &Vec::new(),
    );

    result.pre_assembler.push(PreAssembler::HALT);

    for p in ast.procedurs.iter().rev() {
        if !compile_info.used_procedures.contains(p) {
            continue;
        }

        result.pre_assembler.push(PreAssembler::LABEL(
            procedures_labels.get(p.get_name()).unwrap().clone(),
        ));
        add_commands(
            &p.commands,
            ProcType::Procedure(p.get_name().to_string()),
            &mut result,
            &mut compile_info,
            &mut index_counter,
            &mut label_counter,
            &procedures_labels,
            &ast.procedurs,
            &p.declarations,
            &p.proc_head.params,
        );
        result
            .pre_assembler
            .push(PreAssembler::GET(AbstractVarible::Else(
                AbstractNumber::ProcedureReturn(ProcType::Procedure(p.get_name().to_string())),
            )));
        result
            .pre_assembler
            .push(PreAssembler::INC(AbstractVarible::Else(
                AbstractNumber::Accumulator,
            )));
        result
            .pre_assembler
            .push(PreAssembler::INC(AbstractVarible::Else(
                AbstractNumber::Accumulator,
            )));
        result
            .pre_assembler
            .push(PreAssembler::INC(AbstractVarible::Else(
                AbstractNumber::Accumulator,
            )));
        result
            .pre_assembler
            .push(PreAssembler::JUMPR(AbstractVarible::Else(
                AbstractNumber::Accumulator,
            )));
    }

    (result, compile_info)
}

fn add_commands(
    commands: &Vec<Command>,
    proc_type: ProcType,
    result: &mut Block,
    compile_info: &mut CompileInfo,
    index_counter: &mut Counter,
    label_counter: &mut Counter,
    procedures_labels: &HashMap<String, u64>,
    procedurs: &Vec<Procedure>,
    declarations: &Vec<Declaration>,
    arguments: &Vec<ArgDecl>,
) {
    for d in declarations {
        match d.decl_type {
            DeclType::Number => {
                compile_info.memory.entry(AbstractVarible::Else(AbstractNumber::Var(proc_type.clone(), d.name.clone()))).or_insert_with(|| index_counter.next());
            }
            DeclType::Table(x) => {
                for i in 0..x {
                    compile_info.memory.entry(AbstractVarible::Table(proc_type.clone(), d.name.clone(), AbstractNumber::Const(i))).or_insert_with(|| index_counter.next());
                }
            }
        }
    }

    for com in commands {
        match com {
            Command::Assign(ref x) => match x.expression.clone() {
                Expression::Number(a) => {
                    result
                        .pre_assembler
                        .push(PreAssembler::GET(value_to_abstract_varible(
                            a,
                            proc_type.clone(),
                            arguments,
                        )));
                    result
                        .pre_assembler
                        .push(PreAssembler::PUT(identifier_to_abstract_varible(
                            x.identifier.clone(),
                            proc_type.clone(),
                            arguments,
                        )));
                }
                Expression::Add(a, b) => {
                    result.pre_assembler.push(PreAssembler::GET(value_to_abstract_varible(
                        a,
                        proc_type.clone(),
                        arguments,
                    )));
                    result.pre_assembler.push(PreAssembler::MOVE('h'));

                    result.pre_assembler.push(PreAssembler::GET(value_to_abstract_varible(
                        b,
                        proc_type.clone(),
                        arguments,
                    )));
                    result.pre_assembler.push(PreAssembler::MOVE('g'));

                    result.pre_assembler.push(PreAssembler::ADD);
                    result
                        .pre_assembler
                        .push(PreAssembler::PUT(identifier_to_abstract_varible(
                            x.identifier.clone(),
                            proc_type.clone(),
                            arguments,
                        )));
                }
                Expression::Sub(a, b) => {
                    result.pre_assembler.push(PreAssembler::GET(value_to_abstract_varible(
                        a,
                        proc_type.clone(),
                        arguments,
                    )));
                    result.pre_assembler.push(PreAssembler::MOVE('h'));

                    result.pre_assembler.push(PreAssembler::GET(value_to_abstract_varible(
                        b,
                        proc_type.clone(),
                        arguments,
                    )));
                    result.pre_assembler.push(PreAssembler::MOVE('g'));

                    result.pre_assembler.push(PreAssembler::SUB);
                    result
                        .pre_assembler
                        .push(PreAssembler::PUT(identifier_to_abstract_varible(
                            x.identifier.clone(),
                            proc_type.clone(),
                            arguments,
                        )));
                }
                Expression::Mul(a, b) => {
                    if compile_info.mul_label.is_none() {
                        compile_info.mul_label = Some(label_counter.next());
                        let _ = label_counter.next(); // while inside of mul
                        let _ = label_counter.next(); // if inside of mul
                    }

                    result.pre_assembler.push(PreAssembler::GET(value_to_abstract_varible(
                        a,
                        proc_type.clone(),
                        arguments,
                    )));
                    result.pre_assembler.push(PreAssembler::MOVE('h'));

                    result.pre_assembler.push(PreAssembler::GET(value_to_abstract_varible(
                        b,
                        proc_type.clone(),
                        arguments,
                    )));
                    result.pre_assembler.push(PreAssembler::MOVE('g'));

                    result.pre_assembler.push(PreAssembler::MUL);
                    result
                        .pre_assembler
                        .push(PreAssembler::PUT(identifier_to_abstract_varible(
                            x.identifier.clone(),
                            proc_type.clone(),
                            arguments,
                        )));
                }
                Expression::Div(a, b) => {
                    if compile_info.div_label.is_none() {
                        compile_info.div_label = Some(label_counter.next());
                        let _ = label_counter.next();
                        let _ = label_counter.next();
                        let _ = label_counter.next();
                        let _ = label_counter.next();
                    }

                    result.pre_assembler.push(PreAssembler::GET(value_to_abstract_varible(
                        a,
                        proc_type.clone(),
                        arguments,
                    )));
                    result.pre_assembler.push(PreAssembler::MOVE('h'));

                    result.pre_assembler.push(PreAssembler::GET(value_to_abstract_varible(
                        b,
                        proc_type.clone(),
                        arguments,
                    )));
                    result.pre_assembler.push(PreAssembler::MOVE('g'));

                    result.pre_assembler.push(PreAssembler::DIV);
                    result
                        .pre_assembler
                        .push(PreAssembler::PUT(identifier_to_abstract_varible(
                            x.identifier.clone(),
                            proc_type.clone(),
                            arguments,
                        )));
                }
                Expression::Mod(a, b) => {
                    if compile_info.mod_label.is_none() {
                        compile_info.mod_label = Some(label_counter.next());
                        let _ = label_counter.next();
                        let _ = label_counter.next();
                        let _ = label_counter.next();
                        let _ = label_counter.next();
                        let _ = label_counter.next();
                    }

                    result.pre_assembler.push(PreAssembler::GET(value_to_abstract_varible(
                        a,
                        proc_type.clone(),
                        arguments,
                    )));
                    result.pre_assembler.push(PreAssembler::MOVE('h'));

                    result.pre_assembler.push(PreAssembler::GET(value_to_abstract_varible(
                        b,
                        proc_type.clone(),
                        arguments,
                    )));
                    result.pre_assembler.push(PreAssembler::MOVE('g'));

                    result.pre_assembler.push(PreAssembler::MOD);
                    result
                        .pre_assembler
                        .push(PreAssembler::PUT(identifier_to_abstract_varible(
                            x.identifier.clone(),
                            proc_type.clone(),
                            arguments,
                        )));
                }
            },
            Command::If(ref x) => {
                let mut true_block = Block::new();

                add_commands(
                    &x.then_commands,
                    proc_type.clone(),
                    &mut true_block,
                    compile_info,
                    index_counter,
                    label_counter,
                    procedures_labels,
                    procedurs,
                    declarations,
                    arguments,
                );

                let mut false_block = Block::new();

                add_commands(
                    &x.else_commands,
                    proc_type.clone(),
                    &mut false_block,
                    compile_info,
                    index_counter,
                    label_counter,
                    procedures_labels,
                    procedurs,
                    declarations,
                    arguments,
                );

                condition_to_preassembler(
                    x.condition.clone(),
                    proc_type.clone(),
                    result,
                    index_counter,
                    label_counter,
                    declarations,
                    arguments,
                    &true_block.pre_assembler,
                    &false_block.pre_assembler,
                );
            }
            Command::While(x) => {
                let mut true_block = Block::new();

                let while_label = label_counter.next();

                result.pre_assembler.push(PreAssembler::LABEL(while_label.clone()));

                add_commands(
                    &x.commands,
                    proc_type.clone(),
                    &mut true_block,
                    compile_info,
                    index_counter,
                    label_counter,
                    procedures_labels,
                    procedurs,
                    declarations,
                    arguments,
                );

                true_block.pre_assembler.push(PreAssembler::JUMP(while_label));

                condition_to_preassembler(
                    x.condition.clone(),
                    proc_type.clone(),
                    result,
                    index_counter,
                    label_counter,
                    declarations,
                    arguments,
                    &true_block.pre_assembler,
                    &Vec::new(),
                );
            }
            Command::Repeat(x) => {
                let mut false_block = Block::new();

                let repeat_label = label_counter.next();
                let start_label = label_counter.next();

                result.pre_assembler.push(PreAssembler::JUMP(start_label.clone()));
                result.pre_assembler.push(PreAssembler::LABEL(repeat_label.clone()));

                add_commands(
                    &x.commands,
                    proc_type.clone(),
                    &mut false_block,
                    compile_info,
                    index_counter,
                    label_counter,
                    procedures_labels,
                    procedurs,
                    declarations,
                    arguments,
                );

                false_block.pre_assembler.insert(0, PreAssembler::LABEL(start_label.clone()));
                false_block.pre_assembler.push(PreAssembler::JUMP(repeat_label));

                condition_to_preassembler(
                    x.condition.clone(),
                    proc_type.clone(),
                    result,
                    index_counter,
                    label_counter,
                    declarations,
                    arguments,
                    &Vec::new(),
                    &false_block.pre_assembler,
                );
            }
            Command::Call(x) => {
                let proc = procedurs
                    .iter()
                    .find(|p| p.get_name() == x.name)
                    .cloned()
                    .unwrap();

                compile_info.used_procedures.insert(proc.clone());

                for (arg, param) in x.args.iter().zip(proc.proc_head.params.iter()) {

                    let p = match param.arg_type {
                        ArgType::Number => AbstractVarible::Else(AbstractNumber::Var(
                            ProcType::Procedure(proc.get_name().to_string()),
                            param.name.clone(),
                        )),
                        ArgType::Table => AbstractVarible::Table(
                                ProcType::Procedure(proc.get_name().to_string()),
                                param.name.clone(),
                                AbstractNumber::Const(0),
                            )
                    };

                    let place = compile_info 
                        .memory
                        .entry(p.clone())
                        .or_insert_with(|| index_counter.next())
                        .clone();

                    let p_pointer = match param.arg_type {
                        ArgType::Number => AbstractVarible::Else(AbstractNumber::Pointer(
                            Box::new(AbstractVarible::Else(AbstractNumber::Var(
                                ProcType::Procedure(proc.get_name().to_string()),
                                param.name.clone(),
                            ))),
                        )),
                        ArgType::Table => AbstractVarible::Else(AbstractNumber::Pointer(
                            Box::new(AbstractVarible::Table(
                                ProcType::Procedure(proc.get_name().to_string()),
                                param.name.clone(),
                                AbstractNumber::Const(0),
                            )),
                        )),
                    };

                    let _ = compile_info 
                        .memory
                        .entry(p_pointer.clone())
                        .or_insert(place.clone());

                    if let Some(_) = compile_info.memory.get(&AbstractVarible::Else(AbstractNumber::Pointer(Box::new(AbstractVarible::Else(AbstractNumber::Var(
                            proc_type.clone(),
                            arg.name.clone(),
                        )))))) {
                        result.pre_assembler.push(PreAssembler::GET(AbstractVarible::Else(
                                AbstractNumber::Var(proc_type.clone(), arg.name.clone()),
                        )));
                    } else if let Some(_) = compile_info.memory.get(&AbstractVarible::Else(AbstractNumber::Pointer(Box::new(AbstractVarible::Table(
                            proc_type.clone(),
                            arg.name.clone(),
                            AbstractNumber::Const(0),
                        ))))) {
                        result.pre_assembler.push(PreAssembler::GET(AbstractVarible::Table(
                                proc_type.clone(),
                                arg.name.clone(),
                                AbstractNumber::Const(0),
                        )));
                    } else if let Some(m) = compile_info.memory.get(&AbstractVarible::Table(
                            proc_type.clone(),
                            arg.name.clone(),
                            AbstractNumber::Const(0),)) {
                        result
                            .pre_assembler
                            .push(PreAssembler::GET(AbstractVarible::Else(
                                AbstractNumber::Const(*m),
                            )));
                    } else {
                        let m = compile_info.memory.get(&AbstractVarible::Else(AbstractNumber::Var(
                            proc_type.clone(),
                            arg.name.clone(),
                        ))).unwrap();

                        result
                            .pre_assembler
                            .push(PreAssembler::GET(AbstractVarible::Else(
                                AbstractNumber::Const(*m),
                            )));
                    }

                    result
                        .pre_assembler
                        .push(PreAssembler::PUT(p_pointer.deref().unwrap().clone()));
                }

                let proc_return = AbstractVarible::Else(AbstractNumber::ProcedureReturn(
                    ProcType::Procedure(x.name.clone()),
                ));

                let _ = compile_info
                    .memory
                    .entry(proc_return.clone())
                    .or_insert_with(|| index_counter.next());

                result.pre_assembler.push(PreAssembler::STRK(proc_return));

                result.pre_assembler.push(PreAssembler::JUMP(
                    procedures_labels.get(x.name.as_str()).unwrap().clone(),
                ));
            }
            Command::Read(x) => {
                result.pre_assembler.push(PreAssembler::READ);
                result
                    .pre_assembler
                    .push(PreAssembler::PUT(identifier_to_abstract_varible(
                        x.identifier.clone(),
                        proc_type.clone(),
                        arguments,
                    )));
            }
            Command::Write(x) => {
                result
                    .pre_assembler
                    .push(PreAssembler::GET(value_to_abstract_varible(
                        x.value.clone(),
                        proc_type.clone(),
                        arguments,
                    )));
                result.pre_assembler.push(PreAssembler::WRITE);
            }
        }
    }
}

fn identifier_to_abstract_varible(
    id: Identifier,
    proc_type: ProcType,
    params: &[ArgDecl],
) -> AbstractVarible {
    if params.iter().any(|x| x.name == id.name) {
        match id.indentifier_type {
            IdentifierType::Number => AbstractVarible::Else(AbstractNumber::Pointer(Box::new(
                AbstractVarible::Else(AbstractNumber::Var(proc_type, id.name)),
            ))),
            IdentifierType::TableWithNumber(x) => AbstractVarible::Else(AbstractNumber::Pointer(
                Box::new(AbstractVarible::Table(proc_type, id.name, AbstractNumber::Const(x))),
            )),
            IdentifierType::TableWithIdentifier(x) => {
                if params.iter().any(|xx| xx.name == x) {
                    AbstractVarible::Else(AbstractNumber::Pointer(Box::new(AbstractVarible::Table(
                            proc_type.clone(),
                        id.name,
                        AbstractNumber::Pointer(Box::new(AbstractVarible::Else(AbstractNumber::Var(proc_type.clone(), x))))
                    ))))
                } else {
                    AbstractVarible::Else(AbstractNumber::Pointer(Box::new(AbstractVarible::Table(
                            proc_type.clone(),
                        id.name,
                        AbstractNumber::Var(proc_type, x),
                    ))))
                }
            }
        }
    } else {
        match id.indentifier_type {
            IdentifierType::Number => {
                AbstractVarible::Else(AbstractNumber::Var(proc_type, id.name))
            }
            IdentifierType::TableWithNumber(x) => {
                AbstractVarible::Table(proc_type, id.name, AbstractNumber::Const(x))
            }
            IdentifierType::TableWithIdentifier(x) => {
                if params.iter().any(|xx| xx.name == x) {
                    AbstractVarible::Table(
                            proc_type.clone(),
                        id.name,
                        AbstractNumber::Pointer(Box::new(AbstractVarible::Else(AbstractNumber::Var(proc_type.clone(), x))))
                    )
                } else {
                    AbstractVarible::Table(
                            proc_type.clone(),
                        id.name,
                        AbstractNumber::Var(proc_type, x),
                    )
                }
            }
        }
    }
}

fn value_to_abstract_varible(
    val: Value,
    proc_type: ProcType,
    params: &[ArgDecl],
) -> AbstractVarible {
    match val {
        Value::Number(x) => AbstractVarible::Else(AbstractNumber::Const(x)),
        Value::Identifier(x) => identifier_to_abstract_varible(x, proc_type, params),
    }
}

fn condition_to_preassembler(
    cond: Condition,
    proc_type: ProcType,
    result: &mut Block,
    index_counter: &mut Counter,
    label_counter: &mut Counter,
    declarations: &Vec<Declaration>,
    arguments: &Vec<ArgDecl>,
    true_commands: &Vec<PreAssembler>,
    false_commands: &Vec<PreAssembler>,
) {
    match cond {
        Condition::Equal(a, b) => {
            let false_label = label_counter.next();
            let end_label = label_counter.next();

            result.pre_assembler.push(PreAssembler::GET(value_to_abstract_varible(
                a.clone(),
                proc_type.clone(),
                arguments,
            )));
            result.pre_assembler.push(PreAssembler::MOVE('h'));

            result.pre_assembler.push(PreAssembler::GET(value_to_abstract_varible(
                b.clone(),
                proc_type.clone(),
                arguments,
            )));
            result.pre_assembler.push(PreAssembler::MOVE('g'));

            result.pre_assembler.push(PreAssembler::SUB);

            result.pre_assembler.push(PreAssembler::JPOS(false_label.clone()));

            result.pre_assembler.push(PreAssembler::GET(value_to_abstract_varible(
                a,
                proc_type.clone(),
                arguments,
            )));
            result.pre_assembler.push(PreAssembler::MOVE('g'));

            result.pre_assembler.push(PreAssembler::GET(value_to_abstract_varible(
                b,
                proc_type.clone(),
                arguments,
            )));
            result.pre_assembler.push(PreAssembler::MOVE('h'));

            result.pre_assembler.push(PreAssembler::SUB);

            result.pre_assembler.push(PreAssembler::JPOS(false_label.clone()));
            result.pre_assembler.extend(true_commands.clone());
            result.pre_assembler.push(PreAssembler::JUMP(end_label.clone()));
            result.pre_assembler.push(PreAssembler::LABEL(false_label));
            result.pre_assembler.extend(false_commands.clone());
            result.pre_assembler.push(PreAssembler::LABEL(end_label));
        }
        Condition::NotEqual(a, b) => {
            condition_to_preassembler(
                Condition::Equal(a, b),
                proc_type,
                result,
                index_counter,
                label_counter,
                declarations,
                arguments,
                false_commands,
                true_commands,
            );
        }
        Condition::Less(a, b) => {
            let false_label = label_counter.next();
            let end_label = label_counter.next();

            result.pre_assembler.push(PreAssembler::GET(value_to_abstract_varible(
                a,
                proc_type.clone(),
                arguments,
            )));
            result.pre_assembler.push(PreAssembler::MOVE('g'));

            result.pre_assembler.push(PreAssembler::GET(value_to_abstract_varible(
                b,
                proc_type.clone(),
                arguments,
            )));
            result.pre_assembler.push(PreAssembler::MOVE('h'));

            result.pre_assembler.push(PreAssembler::SUB);

            result.pre_assembler.push(PreAssembler::JZERO(false_label.clone()));
            result.pre_assembler.extend(true_commands.clone());
            result.pre_assembler.push(PreAssembler::JUMP(end_label.clone()));
            result.pre_assembler.push(PreAssembler::LABEL(false_label));
            result.pre_assembler.extend(false_commands.clone());
            result.pre_assembler.push(PreAssembler::LABEL(end_label));
        }
        Condition::LessEqual(a, b) => {
            condition_to_preassembler(
                Condition::Less(b, a),
                proc_type,
                result,
                index_counter,
                label_counter,
                declarations,
                arguments,
                false_commands,
                true_commands,
            );
        }
        Condition::Greater(a, b) => {
            condition_to_preassembler(
                Condition::Less(b, a),
                proc_type,
                result,
                index_counter,
                label_counter,
                declarations,
                arguments,
                true_commands,
                false_commands,
            );
        }
        Condition::GreaterEqual(a, b) => {
            condition_to_preassembler(
                Condition::Less(a, b),
                proc_type,
                result,
                index_counter,
                label_counter,
                declarations,
                arguments,
                false_commands,
                true_commands,
            );
        }
    }
}
