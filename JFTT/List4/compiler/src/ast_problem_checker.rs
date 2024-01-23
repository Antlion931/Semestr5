use std::collections::HashMap;

use crate::ast::*;
use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq)]
enum IdentifierState {
    Initialized,
    PossilbyUninitialized,
}

pub enum Problem {
    Error(ASTError),
    Warning(ASTWarning),
}

pub enum ASTError {
    UndeclaredVarible(Box<dyn DiagnosticInfo>),
    UninitialazedVarible(Box<dyn DiagnosticInfo>),
    WrongUsageOfTable(Box<dyn DiagnosticInfo>),
    WrongUsageOfVariable(Box<dyn DiagnosticInfo>),
    WrongParametersOfAFunction(Box<dyn DiagnosticInfo>),
    IndexOutOfTable(Box<dyn DiagnosticInfo>),
    OverlapingIdentifiers(Box<dyn DiagnosticInfo>),
    FunctionNotDefinedBeforeUsage(Box<dyn DiagnosticInfo>),
    MultipleProceduresWithSameName(Box<dyn DiagnosticInfo>),
}

pub enum ASTWarning {
    UninitialazedVarible(Box<dyn DiagnosticInfo>),
}

pub fn check_for_problems(ast: &AST) -> Vec<Problem> {
    let mut errors = Vec::new();

    let mut known_procedures = Vec::new();

    for proc in &ast.procedurs {
        let mut duplicated_names = Vec::new();

        proc.proc_head
            .params
            .iter()
            .map(|x| &x.name)
            .chain(proc.declarations.iter().map(|x| &x.name))
            .duplicates()
            .for_each(|x| {
                duplicated_names.push(x.clone());
            });

        for p in proc
            .proc_head
            .params
            .iter()
            .filter(|x| duplicated_names.contains(&x.name))
        {
            errors.push(Problem::Error(ASTError::OverlapingIdentifiers(Box::new(
                p.clone(),
            ))));
        }

        let (calls, mut proc_errors) =
            get_function_calls_and_problems_from_commands_given_declarations(
                &proc.commands,
                &ast.procedurs,
                &proc.declarations,
                &proc.proc_head.params,
            );

        errors.append(&mut proc_errors);

        for call in calls {
            if !known_procedures.contains(&call.name) {
                errors.push(Problem::Error(ASTError::FunctionNotDefinedBeforeUsage(
                    Box::new(call),
                )));
            }

        }

        if known_procedures.contains(&proc.proc_head.name) {
            errors.push(Problem::Error(ASTError::MultipleProceduresWithSameName(
                Box::new(proc.proc_head.clone()),
            )));
        }

        known_procedures.push(proc.proc_head.name.clone());
    }

    let mut duplicated_names = Vec::new();

    ast.main
        .declarations
        .iter()
        .map(|x| &x.name)
        .duplicates()
        .for_each(|x| {
            duplicated_names.push(x.clone());
        });

    for p in ast
        .main
        .declarations
        .iter()
        .filter(|x| duplicated_names.contains(&x.name))
    {
        errors.push(Problem::Error(ASTError::OverlapingIdentifiers(Box::new(
            p.clone(),
        ))));
    }

    let (calls, mut main_errors) = get_function_calls_and_problems_from_commands_given_declarations(
        &ast.main.commands,
        &ast.procedurs,
        &ast.main.declarations,
        &[],
    );

    errors.append(&mut main_errors);

    for call in calls {
        if !known_procedures.contains(&call.name) {
            errors.push(Problem::Error(ASTError::FunctionNotDefinedBeforeUsage(
                Box::new(call),
            )));
        }
    }

    errors
}

fn get_function_calls_and_problems_from_commands_given_declarations(
    commands: &[Command],
    procedures: &[Procedure],
    declarations: &[Declaration],
    arguments: &[ArgDecl],
) -> (Vec<Call>, Vec<Problem>) {
    get_function_calls_and_problems_from_commands_given_declarations_rec(
        commands,
        procedures,
        declarations,
        arguments,
        &mut HashMap::new(),
        &mut HashMap::new(),
        0,
    )
}

fn get_function_calls_and_problems_from_commands_given_declarations_rec(
    commands: &[Command],
    procedures: &[Procedure],
    declarations: &[Declaration],
    arguments: &[ArgDecl],
    initialized_variables: &mut HashMap<Identifier, IdentifierState>,
    initialized_tables: &mut HashMap<String, IdentifierState>,
    loop_depth: usize,
) -> (Vec<Call>, Vec<Problem>) {
    let mut calls = Vec::new();
    let mut problems = Vec::new();

    for c in commands {
        match c {
            Command::Call(call) => {
                calls.push(call.clone());

                let mut problem_in_arguments = false;

                if let Some(p) = procedures.iter().find(|x| x.proc_head.name == call.name) {
                    for (arg, name) in p.proc_head.params.iter().zip(call.args.iter().map(|x| &x.name)) {
                        match arg.arg_type {
                            ArgType::Number => {
                                let id = Identifier::new(
                                    name.clone(),
                                    IdentifierType::Number,
                                    call.start,
                                    call.end,
                                );

                                initialized_variables.insert(id.clone(), IdentifierState::Initialized);

                                let mut new_problems = check_if_identifier_is_ok(
                                    &id,
                                    declarations,
                                    arguments,
                                    initialized_variables,
                                    initialized_tables,
                                    loop_depth,
                                );

                                if !new_problems.is_empty() {
                                    problem_in_arguments = true;
                                }

                                problems.append(&mut new_problems);
                            }
                            ArgType::Table => {
                                let id = Identifier::new(
                                    name.clone(),
                                    IdentifierType::TableWithNumber(0),
                                    call.start,
                                    call.end,
                                );

                                initialized_tables.insert(id.name.clone(), IdentifierState::Initialized);

                                let mut new_problems = check_if_identifier_is_ok(
                                    &id,
                                    declarations,
                                    arguments,
                                    initialized_variables,
                                    initialized_tables,
                                    loop_depth,
                                );

                                if !new_problems.is_empty() {
                                    problem_in_arguments = true;
                                }

                                problems.append(&mut new_problems);
                            }
                        }
                    }
                }

                if problem_in_arguments {
                    problems.push(Problem::Error(ASTError::WrongParametersOfAFunction(
                        Box::new(call.clone()),
                    )));
                }
            }
            Command::If(c) => {
                let mut new_problems = problems_in_condition(
                    &c.condition,
                    declarations,
                    arguments,
                    initialized_variables,
                    initialized_tables,
                    loop_depth,
                );
                problems.append(&mut new_problems);

                let (mut new_calls, mut new_problems) =
                    get_function_calls_and_problems_from_commands_given_declarations_rec(
                        &c.then_commands,
                        procedures,
                        declarations,
                        arguments,
                        initialized_variables,
                        initialized_tables,
                        loop_depth,
                    );

                calls.append(&mut new_calls);
                problems.append(&mut new_problems);

                let (mut new_calls, mut new_problems) =
                    get_function_calls_and_problems_from_commands_given_declarations_rec(
                        &c.else_commands,
                        procedures,
                        declarations,
                        arguments,
                        initialized_variables,
                        initialized_tables,
                        loop_depth,
                    );
                calls.append(&mut new_calls);
                problems.append(&mut new_problems);
            }
            Command::While(c) => {
                let mut new_problems = problems_in_condition(
                    &c.condition,
                    declarations,
                    arguments,
                    initialized_variables,
                    initialized_tables,
                    loop_depth,
                );
                problems.append(&mut new_problems);

                let (mut new_calls, mut new_problems) =
                    get_function_calls_and_problems_from_commands_given_declarations_rec(
                        &c.commands,
                        procedures,
                        declarations,
                        arguments,
                        initialized_variables,
                        initialized_tables,
                        loop_depth + 1,
                    );
                calls.append(&mut new_calls);
                problems.append(&mut new_problems);

                for (k, v) in initialized_variables.iter_mut() {
                    if *v == IdentifierState::PossilbyUninitialized {
                        match k.indentifier_type {
                            IdentifierType::Number => {
                                problems.push(Problem::Error(ASTError::UninitialazedVarible(
                                    Box::new(k.clone()),
                                )));
                            }
                            IdentifierType::TableWithNumber(_) => {
                                if IdentifierState::Initialized == *initialized_tables.get(k.name.as_str()).unwrap() {
                                    problems.push(Problem::Warning(ASTWarning::UninitialazedVarible(
                                        Box::new(k.clone()),
                                    )));

                                    *v = IdentifierState::Initialized;
                                } else {
                                    problems.push(Problem::Error(ASTError::UninitialazedVarible(
                                        Box::new(k.clone()),
                                    )));
                                }
                            }
                            IdentifierType::TableWithIdentifier(_) => {
                                if IdentifierState::Initialized == *initialized_tables.get(k.name.as_str()).unwrap() {
                                    problems.push(Problem::Warning(ASTWarning::UninitialazedVarible(
                                        Box::new(k.clone()),
                                    )));

                                    *v = IdentifierState::Initialized;
                                } else {
                                    problems.push(Problem::Error(ASTError::UninitialazedVarible(
                                        Box::new(k.clone()),
                                    )));
                                }
                            }
                        }

                    }
                }


            }
            Command::Repeat(c) => {
                let (mut new_calls, mut new_problems) =
                    get_function_calls_and_problems_from_commands_given_declarations_rec(
                        &c.commands,
                        procedures,
                        declarations,
                        arguments,
                        initialized_variables,
                        initialized_tables,
                        loop_depth + 1,
                    );
                calls.append(&mut new_calls);
                problems.append(&mut new_problems);

                let mut new_problems = problems_in_condition(
                    &c.condition,
                    declarations,
                    arguments,
                    initialized_variables,
                    initialized_tables,
                    loop_depth,
                );
                problems.append(&mut new_problems);

            for (k, v) in initialized_variables.iter_mut() {
                    if *v == IdentifierState::PossilbyUninitialized {
                        match k.indentifier_type {
                            IdentifierType::Number => {
                                problems.push(Problem::Error(ASTError::UninitialazedVarible(
                                    Box::new(k.clone()),
                                )));
                            }
                            IdentifierType::TableWithNumber(_) => {
                                if IdentifierState::Initialized == *initialized_tables.get(k.name.as_str()).unwrap() {
                                    problems.push(Problem::Warning(ASTWarning::UninitialazedVarible(
                                        Box::new(k.clone()),
                                    )));

                                    *v = IdentifierState::Initialized;
                                } else {
                                    problems.push(Problem::Error(ASTError::UninitialazedVarible(
                                        Box::new(k.clone()),
                                    )));
                                }
                            }
                            IdentifierType::TableWithIdentifier(_) => {
                                if IdentifierState::Initialized == *initialized_tables.get(k.name.as_str()).unwrap() {
                                    problems.push(Problem::Warning(ASTWarning::UninitialazedVarible(
                                        Box::new(k.clone()),
                                    )));

                                    *v = IdentifierState::Initialized;
                                } else {
                                    problems.push(Problem::Error(ASTError::UninitialazedVarible(
                                        Box::new(k.clone()),
                                    )));
                                }
                            }
                        }

                    }
                }

            }
            Command::Assign(assign) => {
                let id = assign.identifier.clone();

                match id.indentifier_type {
                    IdentifierType::Number => {
                        initialized_variables.insert(id.clone(), IdentifierState::Initialized);
                    }
                    IdentifierType::TableWithNumber(_) => {
                        initialized_variables.insert(id.clone(), IdentifierState::Initialized);
                    }
                    IdentifierType::TableWithIdentifier(_) => {
                        initialized_tables.insert(id.name.clone(), IdentifierState::Initialized);
                    }
                }

                let mut new_problems = check_if_identifier_is_ok(
                    &id,
                    declarations,
                    arguments,
                    initialized_variables,
                    initialized_tables,
                    loop_depth,
                );
                problems.append(&mut new_problems);

                match &assign.expression {
                    Expression::Number(a) => {
                        if let Value::Identifier(id) = a {
                            let mut new_problems = check_if_identifier_is_ok(
                                &id,
                                declarations,
                                arguments,
                                initialized_variables,
                                initialized_tables,
                                loop_depth,
                            );
                            problems.append(&mut new_problems);
                        }
                    }
                    Expression::Add(a, b) => {
                        if let Value::Identifier(id) = a {
                            let mut new_problems = check_if_identifier_is_ok(
                                &id,
                                declarations,
                                arguments,
                                initialized_variables,
                                initialized_tables,
                                loop_depth,
                            );
                            problems.append(&mut new_problems);
                        }

                        if let Value::Identifier(id) = b {
                            let mut new_problems = check_if_identifier_is_ok(
                                &id,
                                declarations,
                                arguments,
                                initialized_variables,
                                initialized_tables,
                                loop_depth,
                            );
                            problems.append(&mut new_problems);
                        }
                    }
                    Expression::Sub(a, b) => {
                        if let Value::Identifier(id) = a {
                            let mut new_problems = check_if_identifier_is_ok(
                                &id,
                                declarations,
                                arguments,
                                initialized_variables,
                                initialized_tables,
                                loop_depth,
                            );
                            problems.append(&mut new_problems);
                        }

                        if let Value::Identifier(id) = b {
                            let mut new_problems = check_if_identifier_is_ok(
                                &id,
                                declarations,
                                arguments,
                                initialized_variables,
                                initialized_tables,
                                loop_depth,
                            );
                            problems.append(&mut new_problems);
                        }
                    }
                    Expression::Mul(a, b) => {
                        if let Value::Identifier(id) = a {
                            let mut new_problems = check_if_identifier_is_ok(
                                &id,
                                declarations,
                                arguments,
                                initialized_variables,
                                initialized_tables,
                                loop_depth,
                            );
                            problems.append(&mut new_problems);
                        }

                        if let Value::Identifier(id) = b {
                            let mut new_problems = check_if_identifier_is_ok(
                                &id,
                                declarations,
                                arguments,
                                initialized_variables,
                                initialized_tables,
                                loop_depth,
                            );
                            problems.append(&mut new_problems);
                        }
                    }
                    Expression::Div(a, b) => {
                        if let Value::Identifier(id) = a {
                            let mut new_problems = check_if_identifier_is_ok(
                                &id,
                                declarations,
                                arguments,
                                initialized_variables,
                                initialized_tables,
                                loop_depth,
                            );
                            problems.append(&mut new_problems);
                        }

                        if let Value::Identifier(id) = b {
                            let mut new_problems = check_if_identifier_is_ok(
                                &id,
                                declarations,
                                arguments,
                                initialized_variables,
                                initialized_tables,
                                loop_depth,
                            );
                            problems.append(&mut new_problems);
                        }
                    }
                    Expression::Mod(a, b) => {
                        if let Value::Identifier(id) = a {
                            let mut new_problems = check_if_identifier_is_ok(
                                &id,
                                declarations,
                                arguments,
                                initialized_variables,
                                initialized_tables,
                                loop_depth,
                            );
                            problems.append(&mut new_problems);
                        }

                        if let Value::Identifier(id) = b {
                            let mut new_problems = check_if_identifier_is_ok(
                                &id,
                                declarations,
                                arguments,
                                initialized_variables,
                                initialized_tables,
                                loop_depth,
                            );
                            problems.append(&mut new_problems);
                        }
                    }
                }
            }
            Command::Read(read) => {
                let id = read.identifier.clone();

                match id.indentifier_type {
                    IdentifierType::Number => {
                        initialized_variables.insert(id.clone(), IdentifierState::Initialized);
                    }
                    IdentifierType::TableWithNumber(_) => {
                        initialized_variables.insert(id.clone(), IdentifierState::Initialized);
                    }
                    IdentifierType::TableWithIdentifier(_) => {
                        initialized_tables.insert(id.name.clone(), IdentifierState::Initialized);
                    }
                }

                let mut new_problems = check_if_identifier_is_ok(
                    &id,
                    declarations,
                    arguments,
                    initialized_variables,
                    initialized_tables,
                    loop_depth,
                );
                problems.append(&mut new_problems);
            }
            Command::Write(write) => {
                if let Value::Identifier(id) = &write.value {
                    let mut new_problems = check_if_identifier_is_ok(
                        &id,
                        declarations,
                        arguments,
                        initialized_variables,
                        initialized_tables,
                        loop_depth,
                    );
                    problems.append(&mut new_problems);
                }
            }
        }
    }

    (calls, problems)
}

fn check_if_identifier_is_ok(
    id: &Identifier,
    declarations: &[Declaration],
    arguments: &[ArgDecl],
    initialized_variables: &mut HashMap<Identifier, IdentifierState>,
    initialized_tables: &mut HashMap<String, IdentifierState>,
    loop_depth: usize,
) -> Vec<Problem> {
    let mut problems = Vec::new();

    let mut found = false;

    if let Some(d) = declarations.iter().find(|x| x.name == id.name) {
        found = true;
        match &id.indentifier_type {
            IdentifierType::Number => match d.decl_type {
                DeclType::Number => {
                    if !initialized_variables.contains_key(id) {
                        if loop_depth != 0 {
                            initialized_variables.insert(
                                id.clone(),
                                IdentifierState::PossilbyUninitialized,
                            );
                        } else {
                            problems.push(Problem::Error(ASTError::UninitialazedVarible(
                                Box::new(id.clone()),
                            )));
                        }
                    }
                }
                DeclType::Table(_) => {
                    problems.push(Problem::Error(ASTError::WrongUsageOfTable(Box::new(
                        id.clone(),
                    ))));
                }
            },
            IdentifierType::TableWithNumber(x) => match d.decl_type {
                DeclType::Number => {
                    problems.push(Problem::Error(ASTError::WrongUsageOfVariable(Box::new(
                        id.clone(),
                    ))));
                }
                DeclType::Table(i) => {
                    if !initialized_tables.contains_key(&id.name) {
                        if !initialized_variables.contains_key(id) {
                            if loop_depth != 0 {
                                initialized_variables.insert(
                                    id.clone(),
                                    IdentifierState::PossilbyUninitialized,
                                );
                            } else {
                                problems.push(Problem::Error(ASTError::UninitialazedVarible(
                                    Box::new(id.clone()),
                                )));
                            }
                        }
                    }

                    if *x >= i {
                        problems.push(Problem::Error(ASTError::IndexOutOfTable(Box::new(
                            id.clone(),
                        ))));
                    }
                }
            },
            IdentifierType::TableWithIdentifier(x) => match d.decl_type {
                DeclType::Number => {
                    problems.push(Problem::Error(ASTError::WrongUsageOfVariable(Box::new(
                        id.clone(),
                    ))));
                }
                DeclType::Table(_) => {
                    let mut new_problems = check_if_identifier_is_ok(
                        &Identifier::new(x.to_owned(), IdentifierType::Number, id.start, id.end),
                        declarations,
                        arguments,
                        initialized_variables,
                        initialized_tables,
                        loop_depth,
                    );
                    problems.append(&mut new_problems);
                }
            },
        }
    }

    if let Some(a) = arguments.iter().find(|x| x.name == id.name) {
        found = true;
        match &id.indentifier_type {
            IdentifierType::Number => match a.arg_type {
                ArgType::Table => {
                    problems.push(Problem::Error(ASTError::WrongUsageOfTable(Box::new(
                        id.clone(),
                    ))));
                }
                _ => {}
            },
            IdentifierType::TableWithNumber(_) => match a.arg_type {
                ArgType::Number => {
                    problems.push(Problem::Error(ASTError::WrongUsageOfVariable(Box::new(
                        id.clone(),
                    ))));
                }
                _ => {}
            },
            IdentifierType::TableWithIdentifier(x) => match a.arg_type {
                ArgType::Number => {
                    problems.push(Problem::Error(ASTError::WrongUsageOfVariable(Box::new(
                        id.clone(),
                    ))));
                }
                ArgType::Table => {
                    let mut new_problems = check_if_identifier_is_ok(
                        &Identifier::new(x.to_owned(), IdentifierType::Number, id.start, id.end),
                        declarations,
                        arguments,
                        initialized_variables,
                        initialized_tables,
                        loop_depth,
                    );
                    problems.append(&mut new_problems);
                }
            },
        }
    }

    if !found {
        problems.push(Problem::Error(ASTError::UndeclaredVarible(Box::new(
            id.clone(),
        ))));
    }

    problems
}

fn problems_in_condition(
    condition: &Condition,
    declarations: &[Declaration],
    arguments: &[ArgDecl],
    initialized_variables: &mut HashMap<Identifier, IdentifierState>,
    initialized_tables: &mut HashMap<String, IdentifierState>,
    loop_depth: usize,
) -> Vec<Problem> {
    let mut problems = Vec::new();

    match condition {
        Condition::Equal(a, b) => {
            if let Value::Identifier(id) = a {
                let mut new_problems = check_if_identifier_is_ok(
                    &id,
                    declarations,
                    arguments,
                    initialized_variables,
                    initialized_tables,
                    loop_depth,
                );
                problems.append(&mut new_problems);
            }

            if let Value::Identifier(id) = b {
                let mut new_problems = check_if_identifier_is_ok(
                    &id,
                    declarations,
                    arguments,
                    initialized_variables,
                    initialized_tables,
                    loop_depth,
                );
                problems.append(&mut new_problems);
            }
        }
        Condition::NotEqual(a, b) => {
            if let Value::Identifier(id) = a {
                let mut new_problems = check_if_identifier_is_ok(
                    &id,
                    declarations,
                    arguments,
                    initialized_variables,
                    initialized_tables,
                    loop_depth,
                );
                problems.append(&mut new_problems);
            }

            if let Value::Identifier(id) = b {
                let mut new_problems = check_if_identifier_is_ok(
                    &id,
                    declarations,
                    arguments,
                    initialized_variables,
                    initialized_tables,
                    loop_depth,
                );
                problems.append(&mut new_problems);
            }
        }
        Condition::Less(a, b) => {
            if let Value::Identifier(id) = a {
                let mut new_problems = check_if_identifier_is_ok(
                    &id,
                    declarations,
                    arguments,
                    initialized_variables,
                    initialized_tables,
                    loop_depth,
                );
                problems.append(&mut new_problems);
            }

            if let Value::Identifier(id) = b {
                let mut new_problems = check_if_identifier_is_ok(
                    &id,
                    declarations,
                    arguments,
                    initialized_variables,
                    initialized_tables,
                    loop_depth,
                );
                problems.append(&mut new_problems);
            }
        }
        Condition::LessEqual(a, b) => {
            let mut new_problems = problems_in_condition(
                &Condition::Less(a.clone(), b.clone()),
                declarations,
                arguments,
                initialized_variables,
                initialized_tables,
                loop_depth,
            );
            problems.append(&mut new_problems);
        }
        Condition::Greater(a, b) => {
            let mut new_problems = problems_in_condition(
                &Condition::Less(a.clone(), b.clone()),
                declarations,
                arguments,
                initialized_variables,
                initialized_tables,
                loop_depth,
            );
            problems.append(&mut new_problems);
        }
        Condition::GreaterEqual(a, b) => {
            let mut new_problems = problems_in_condition(
                &Condition::Less(a.clone(), b.clone()),
                declarations,
                arguments,
                initialized_variables,
                initialized_tables,
                loop_depth,
            );
            problems.append(&mut new_problems);
        }
    }
    
    problems
}
