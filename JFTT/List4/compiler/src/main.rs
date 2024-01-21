mod ast;
mod pre_assembler;
mod ast_to_pre_assembler;
mod ast_problem_checker;
mod pre_assembler_to_assembler;

use lalrpop_util::lalrpop_mod;
use std::fs;
use ast_problem_checker::*;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term;

lalrpop_mod!(pub parser);

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <filename>", args[0]);
        std::process::exit(1);
    }

    let code = fs::read_to_string(&args[1]).expect("Error reading file");

    let mut code_without_comments = String::new();

    for line in code.lines() {
        for c in line.chars() {
            if c == '#' {
                break;
            }
            code_without_comments.push(c);
        }
        code_without_comments.push('\n');
    }

    let ast = parser::program_allParser::new().parse(&code_without_comments).unwrap();

    let mut files = SimpleFiles::new();
    let file_id = files.add(&args[1], &code_without_comments);

    let problems = check_for_problems(&ast);
    let mut diagnostics = Vec::new();

    let mut therer_was_error = false;

    if !problems.is_empty() {
        for p in problems {
            match p {
                Problem::Error(e) => {
                    therer_was_error = true;
                    match e {
                        ASTError::OverlapingIdentifiers(d) => {
                            diagnostics.push(Diagnostic::error()
                                .with_message("There is more than one variable with the same name.")
                                .with_labels(vec![
                                    Label::primary(file_id, d.get_start()..d.get_end()).with_message(format!("There is other variable with the name {}.", d.get_name())),
                                ]));
                        }
                        ASTError::FunctionNotDefinedBeforeUsage(d) => {
                            diagnostics.push(Diagnostic::error()
                                .with_message("Procedure is used before it is defined.")
                                .with_labels(vec![
                                    Label::primary(file_id, d.get_start()..d.get_end()).with_message(format!("There is no definition for {} before it first use.", d.get_name())),
                                ]));
                        }
                        _ => {
                            println!("Error: Unknown error");
                        }
                    }
                }
                Problem::Warning(w) => {
                    match w {
                        ASTWarning::UninitialazedVarible(d) => {
                            diagnostics.push(Diagnostic::warning()
                                .with_message("Variable could be used before initialization.")
                                .with_labels(vec![
                                    Label::primary(file_id, d.get_start()..d.get_end()).with_message(format!("Variable {} could be used before initialization.", d.get_name())),
                                ]));
                        }
                    }
                }
            }
        }
    }

    let writer = StandardStream::stderr(ColorChoice::Always);
    let config = codespan_reporting::term::Config::default();

    for d in diagnostics {
        term::emit(&mut writer.lock(), &config, &files, &d).expect("Error writing diagnostic");
    }

    if therer_was_error {
        std::process::exit(1);
    }

    let pre_assembler = ast_to_pre_assembler::ast_to_pre_assembler(ast);
    let assembler = pre_assembler_to_assembler::pre_assembler_to_assembler(pre_assembler);

    println!("{assembler}");
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Write;
    use std::fs::File;
    use std::process;

    fn tester(code: &str, path: &str, input: &[u64], expected_output: &[u128]) {
        let ast = parser::program_allParser::new().parse(&code).unwrap();

        let pre_assembler = ast_to_pre_assembler::ast_to_pre_assembler(ast);
        let assembler = pre_assembler_to_assembler::pre_assembler_to_assembler(pre_assembler);

        let mut compiled = File::create(path).unwrap();

        write!(compiled, "{}", assembler).unwrap();

        let mut vm = process::Command::new("./../maszyna_wirtualna/maszyna-wirtualna")
            .arg(path)
            .stdout(process::Stdio::piped())
            .stdin(process::Stdio::piped())
            .spawn()
            .unwrap();

        let mut i = vm.stdin.take().unwrap();

        for c in input {
            write!(i, "{}\n", c).unwrap();
        }

        let output = vm.wait_with_output().unwrap();

        let o = String::from_utf8_lossy(&output.stdout);

        let o = o.chars().filter(|c| *c != '>' && *c != '?').collect::<String>();

        let numbers = o.lines().map(|l| l.trim()).filter_map(|l| u128::from_str_radix(l, 10).ok()).collect::<Vec<_>>();

        assert_eq!( numbers, expected_output);
    }

    #[test]
    fn simple_read_write() {
        let code = "PROGRAM IS
a, b, c, d
IN
READ a ;
READ b ;
READ c ;
READ d ;
WRITE d;
WRITE c;
WRITE b;
WRITE a;
END
";
        tester(code, "test1.txt", &[1, 2, 3, 4], &[4, 3, 2, 1]);
    }
}
