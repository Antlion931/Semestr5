use crate::ast::*;
use itertools::Itertools;
use crate::ast::*;

pub enum Problem {
    Error(ASTError),
    Warning(ASTWarning),
}

pub enum ASTError {
    UndeclaredVarible(Box<dyn DiagnosticInfo>),
    UninitialazedVarible(Box<dyn DiagnosticInfo>),
    WrongUsageOfTable(Box<dyn DiagnosticInfo>),
    WrongParametersOfAFunction(Box<dyn DiagnosticInfo>),
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

        proc.proc_head.params.iter().map(|x| &x.name).chain(proc.declarations.iter().map(|x| &x.name)).duplicates().for_each(|x| {
            duplicated_names.push(x.clone());
        });

        for p in proc.proc_head.params.iter().filter(|x| duplicated_names.contains(&x.name)) {
            errors.push(Problem::Error(ASTError::OverlapingIdentifiers(Box::new(p.clone()))));
        }

        let varibles = proc.proc_head.params.iter().filter_map(|x| match x.arg_type {
            ArgType::Number => Some(x.name.as_str()),
            _ => None,
        }).chain(proc.declarations.iter().filter_map(|x| match x.decl_type {
            DeclType::Number => Some(x.name.as_str()),
            _ => None,
        })).collect_vec();

        let tables = proc.proc_head.params.iter().filter_map(|x| match x.arg_type {
            ArgType::Table => Some(x.name.as_str()),
            _ => None,
        }).chain(proc.declarations.iter().filter_map(|x| match x.decl_type {
            DeclType::Table(_) => Some(x.name.as_str()),
            _ => None,
        })).collect_vec();

        let (calls, mut proc_errors) = get_function_calls_and_problems_from_commands_given_declarations(&proc.commands, &tables, &varibles);

        errors.append(&mut proc_errors);

        for call in calls {
            if !known_procedures.contains(&call.name) {
                errors.push(Problem::Error(ASTError::FunctionNotDefinedBeforeUsage(Box::new(call))));
            }
        }

        if known_procedures.contains(&proc.proc_head.name) {
            errors.push(Problem::Error(ASTError::MultipleProceduresWithSameName(Box::new(proc.proc_head.clone()))));
        }

        known_procedures.push(proc.proc_head.name.clone());
    }

    let mut duplicated_names = Vec::new();

    ast.main.declarations.iter().map(|x| &x.name).duplicates().for_each(|x| {
        duplicated_names.push(x.clone());
    });

    for p in ast.main.declarations.iter().filter(|x| duplicated_names.contains(&x.name)) {
        errors.push(Problem::Error(ASTError::OverlapingIdentifiers(Box::new(p.clone()))));
    }

    let varibles = ast.main.declarations.iter().filter_map(|x| match x.decl_type {
        DeclType::Number => Some(x.name.as_str()),
        _ => None,
    }).collect_vec();

    let tables = ast.main.declarations.iter().filter_map(|x| match x.decl_type {
        DeclType::Table(_) => Some(x.name.as_str()),
        _ => None,
    }).collect_vec();

    let (calls, mut main_errors) = get_function_calls_and_problems_from_commands_given_declarations(&ast.main.commands, &tables, &varibles);

    errors.append(&mut main_errors);

    for call in calls {
        if !known_procedures.contains(&call.name) {
            errors.push(Problem::Error(ASTError::FunctionNotDefinedBeforeUsage(Box::new(call))));
        }
    }

    errors
}

fn get_function_calls_and_problems_from_commands_given_declarations(commands: &[Command], tables: &[&str], varibles: &[&str]) -> (Vec<Call>, Vec<Problem>) {
    let mut calls = Vec::new();
    let mut errors = Vec::new();

    for c in commands {
        match c {
            Command::Call(call) => {
                calls.push(call.clone());
            }
            Command::If(command) => {
            }
            Command::While(while_command) => {
            }
            Command::Repeat(repeat_command) => {
            }
            Command::Assign(assign) => {
            }
            Command::Read(read) => {
            }
            Command::Write(write) => {
            }
        }
    }

    (calls, errors)
}
