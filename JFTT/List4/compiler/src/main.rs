mod ast;
mod ast_problem_checker;
mod ast_to_pre_assembler;
mod common;
mod pre_assembler;
mod pre_assembler_to_assembler;

use ast_problem_checker::*;
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use lalrpop_util::lalrpop_mod;
use std::fs;

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

    let ast = parser::program_allParser::new()
        .parse(&code_without_comments)
        .unwrap();

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
                            diagnostics.push(
                                Diagnostic::error()
                                    .with_message(
                                        "There is more than one variable with the same name.",
                                    )
                                    .with_labels(vec![Label::primary(
                                        file_id,
                                        d.get_start()..d.get_end(),
                                    )
                                    .with_message(format!(
                                        "There is other variable with the name {}.",
                                        d.get_name()
                                    ))]),
                            );
                        }
                        ASTError::FunctionNotDefinedBeforeUsage(d) => {
                            diagnostics.push(
                                Diagnostic::error()
                                    .with_message("Procedure is used before it is defined.")
                                    .with_labels(vec![Label::primary(
                                        file_id,
                                        d.get_start()..d.get_end(),
                                    )
                                    .with_message(format!(
                                        "There is no definition for {} before it first use.",
                                        d.get_name()
                                    ))]),
                            );
                        }
                        ASTError::MultipleProceduresWithSameName(d) => {
                            diagnostics.push(
                                Diagnostic::error()
                                    .with_message(
                                        "There is more than one procedure with the same name.",
                                    )
                                    .with_labels(vec![Label::primary(
                                        file_id,
                                        d.get_start()..d.get_end(),
                                    )
                                    .with_message(format!(
                                        "There is other procedure with the name {}.",
                                        d.get_name()
                                    ))]),
                            );
                        }
                        ASTError::UndeclaredVarible(d) => {
                            diagnostics.push(
                                Diagnostic::error()
                                    .with_message("Variable is not declared declared.")
                                    .with_labels(vec![Label::primary(
                                        file_id,
                                        d.get_start()..d.get_end(),
                                    )
                                    .with_message(format!(
                                        "There is no declaration for {} before it first use.",
                                        d.get_name()
                                    ))]),
                            );
                        }
                        ASTError::UninitialazedVarible(d) => {
                            diagnostics.push(
                                Diagnostic::error()
                                    .with_message("Variable is used before initialization.")
                                    .with_labels(vec![Label::primary(
                                        file_id,
                                        d.get_start()..d.get_end(),
                                    )
                                    .with_message(format!(
                                        "Variable {} is used before initialization.",
                                        d.get_name()
                                    ))]),
                            );
                        }
                        ASTError::WrongUsageOfTable(d) => {
                            diagnostics.push(
                                Diagnostic::error()
                                    .with_message("Wrong usage of table.")
                                    .with_labels(vec![Label::primary(
                                        file_id,
                                        d.get_start()..d.get_end(),
                                    )
                                    .with_message(format!(
                                        "Wrong usage of table {}.",
                                        d.get_name()
                                    ))]),
                            );
                        }
                        ASTError::WrongUsageOfVariable(d) => {
                            diagnostics.push(
                                Diagnostic::error()
                                    .with_message("Wrong usage of variable.")
                                    .with_labels(vec![Label::primary(
                                        file_id,
                                        d.get_start()..d.get_end(),
                                    )
                                    .with_message(format!(
                                        "Wrong usage of variable {}.",
                                        d.get_name()
                                    ))]),
                            );
                        }
                        ASTError::WrongParametersOfAFunction(d) => {
                            diagnostics.push(
                                Diagnostic::error()
                                    .with_message("Wrong parameters of a function.")
                                    .with_labels(vec![Label::primary(
                                        file_id,
                                        d.get_start()..d.get_end(),
                                    )
                                    .with_message(format!(
                                        "Wrong parameters of a function {}.",
                                        d.get_name()
                                    ))]),
                            );
                        }
                        ASTError::IndexOutOfTable(d) => {
                            diagnostics.push(
                                Diagnostic::error()
                                    .with_message("Index out of table.")
                                    .with_labels(vec![Label::primary(
                                        file_id,
                                        d.get_start()..d.get_end(),
                                    )
                                    .with_message(format!(
                                        "Index out of table {}.",
                                        d.get_name()
                                    ))]),
                            );
                        }
                    }
                }
                Problem::Warning(w) => match w {
                    ASTWarning::UninitialazedVarible(d) => {
                        diagnostics.push(
                            Diagnostic::warning()
                                .with_message("Variable could be used before initialization.")
                                .with_labels(vec![Label::primary(
                                    file_id,
                                    d.get_start()..d.get_end(),
                                )
                                .with_message(format!(
                                    "Variable {} could be used before initialization.",
                                    d.get_name()
                                ))]),
                        );
                    }
                },
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
    use std::fs::File;
    use std::io::Write;
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

        let o = o
            .chars()
            .filter(|c| *c != '>' && *c != '?')
            .collect::<String>();

        let numbers = o
            .lines()
            .map(|l| l.trim())
            .filter_map(|l| u128::from_str_radix(l, 10).ok())
            .collect::<Vec<_>>();

        assert_eq!(numbers, expected_output);
    }

    fn file_tester(in_path: &str, compiled_path: &str, input: &[u64], expected_output: &[u128]) {
        let code = fs::read_to_string(in_path).expect("Error reading file");

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

        let pre_assembler = ast_to_pre_assembler::ast_to_pre_assembler(ast);

        println!("{:#?}", pre_assembler.memory);

        let assembler = pre_assembler_to_assembler::pre_assembler_to_assembler(pre_assembler);

        let mut compiled = File::create(compiled_path).unwrap();

        write!(compiled, "{}", assembler).unwrap();

        let mut vm = process::Command::new("./../maszyna_wirtualna/maszyna-wirtualna")
            .arg(compiled_path)
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

        let o = o
            .chars()
            .filter(|c| *c != '>' && *c != '?')
            .collect::<String>();

        let numbers = o
            .lines()
            .map(|l| l.trim())
            .filter_map(|l| u128::from_str_radix(l, 10).ok())
            .collect::<Vec<_>>();

        assert_eq!(numbers, expected_output);
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
        tester(code, "simple_read_write.txt", &[1, 2, 3, 4], &[4, 3, 2, 1]);
    }

    #[test]
    fn simple_add() {
        let code = "PROGRAM IS
a, b
IN
READ a ;
READ b ;
a := a + b;
WRITE a;
END
";

        tester(code, "simple_add.txt", &[1, 2], &[3]);
    }

    #[test]
    fn simple_assign() {
        let code = "PROGRAM IS
pi
IN
pi := 314;
WRITE pi;
END";

        tester(code, "simple_assign.txt", &[], &[314]);
    }

    #[test]
    fn all_simple_adds() {
        let code = "PROGRAM IS
a, b, c
IN
READ a ;
READ b ;
c := a + b;
WRITE c;
c := a + 10;
WRITE c;
c := 10 + b;
WRITE c;
c := 10 + 10;
WRITE c;
END";

        tester(code, "all_simple_adds.txt", &[1, 2], &[3, 11, 12, 20]);
    }

    #[test]
    fn all_simple_subs() {
        let code = "PROGRAM IS
a, b, c
IN
READ a ;
READ b ;
c := a - b;
WRITE c;
c := a - 10;
WRITE c;
c := 10 - b;
WRITE c;
c := 10 - 10;
WRITE c;
END";

        tester(code, "all_simple_subs.txt", &[13, 2], &[11, 3, 8, 0]);
    }

    #[test]
    fn procedure_test() {
        let code = "
PROCEDURE add(a, b) IS
c
IN
c := a + b;
WRITE c;
END

PROCEDURE sub(a, b) IS
c
IN
c := a - b;
WRITE c;
END

PROGRAM IS
a, b
IN
READ a ;
READ b ;
add(a, b);
sub(a, b);
END";

        tester(code, "procedure_test.txt", &[13, 2], &[15, 11]);
    }

    #[test]
    fn simple_if_eq() {
        let code = "PROGRAM IS
a, b
IN
READ a;
READ b;
IF a = b THEN
WRITE 1;
ELSE
WRITE 0;
ENDIF
END";

        tester(code, "simple_if_eq_1.txt", &[1, 1], &[1]);
        tester(code, "simple_if_eq_2.txt", &[1, 2], &[0]);
    }

    #[test]
    fn all_conditions_in_if() {
        let code = "PROGRAM IS
a, b
IN
READ a;
READ b;

IF a = b THEN
    WRITE 1;
ELSE
    WRITE 0;
ENDIF

IF a != b THEN
    WRITE 1;
ELSE
    WRITE 0;
ENDIF

IF a < b THEN
    WRITE 1;
ELSE
    WRITE 0;
ENDIF

IF a <= b THEN
    WRITE 1;
ELSE
    WRITE 0;
ENDIF

IF a > b THEN
    WRITE 1;
ELSE
    WRITE 0;
ENDIF

IF a >= b THEN
    WRITE 1;
ELSE
    WRITE 0;
ENDIF
    
END";
    
            tester(code, "all_conditions_in_if_1.txt", &[1, 1], &[1, 0, 0, 1, 0, 1]);
            tester(code, "all_conditions_in_if_2.txt", &[1, 2], &[0, 1, 1, 1, 0, 0]);
            tester(code, "all_conditions_in_if_3.txt", &[2, 1], &[0, 1, 0, 0, 1, 1]);
        }

    #[test]
    fn eq_in_while() {
        let code = "PROGRAM IS
a, b
IN
READ a;
READ b;
WHILE a = b DO
    WRITE 1;
    READ a;
    READ b;
ENDWHILE
    END";

        tester(code, "eq_in_while.txt_1", &[1, 1, 2, 2, 3, 3, 4, 5], &[1, 1, 1]);
        tester(code, "eq_in_while.txt_2", &[1, 2], &[]);
    }

    #[test]
    fn not_eq_in_while() {
        let code = "PROGRAM IS
a, b
IN
READ a;
READ b;
WHILE a != b DO
    WRITE 1;
    READ a;
    READ b;
ENDWHILE
END";

        tester(code, "not_eq_in_while_1.txt", &[1, 1, 2, 2, 3, 3, 4, 5], &[]);
        tester(code, "not_eq_in_while_2.txt", &[1, 2, 3, 1, 2, 2], &[1, 1]);
    }

    #[test]
    fn lt_in_while() {
        let code = "PROGRAM IS
a, b
IN
READ a;
READ b;
WHILE a < b DO
    WRITE 1;
    READ a;
    READ b;
ENDWHILE
END";

        tester(code, "lt_in_while_1.txt", &[1, 1, 2, 2, 3, 3, 5, 4], &[]);
        tester(code, "lt_in_while_2.txt", &[1, 2, 3, 6, 3, 1], &[1, 1]);
    }

    #[test]
    fn le_in_while() {
        let code = "PROGRAM IS
a, b
IN
READ a;
READ b;
WHILE a <= b DO
    WRITE 1;
    READ a;
    READ b;
ENDWHILE
END";

        tester(code, "le_in_while_1.txt", &[1, 1, 2, 2, 3, 3, 5, 4], &[1, 1, 1]);
        tester(code, "le_in_while_2.txt", &[1, 2, 3, 6, 3, 1], &[1, 1]);
    }

    #[test]
    fn gr_in_while() {
        let code = "PROGRAM IS
a, b
IN
READ a;
READ b;
WHILE a > b DO
    WRITE 1;
    READ a;
    READ b;
ENDWHILE
END";

        tester(code, "gr_in_while_1.txt", &[1, 1, 2, 2, 3, 3, 4, 5], &[]);
        tester(code, "gr_in_while_2.txt", &[2, 1, 6, 3, 1, 3], &[1, 1]);
    }

    #[test]
    fn ge_in_while() {
        let code = "PROGRAM IS
a, b
IN
READ a;
READ b;
WHILE a >= b DO
    WRITE 1;
    READ a;
    READ b;
ENDWHILE
END";

        tester(code, "ge_in_while_1.txt", &[1, 1, 2, 2, 3, 3, 4, 5], &[1, 1, 1]);
        tester(code, "ge_in_while_2.txt", &[2, 1, 6, 3, 1, 3], &[1, 1]);
    }

    #[test]
    fn eq_in_repeat() {
        let code = "PROGRAM IS
a, b
IN
REPEAT
    WRITE 1;
    READ a;
    READ b;
UNTIL a = b;
END";

        tester(code, "eq_in_repeat_1.txt", &[1, 1, 2, 2, 3, 3, 4, 5], &[1, 1, 1, 1]);
        tester(code, "eq_in_repeat_2.txt", &[1, 2], &[1]);
    }

    #[test]
    fn not_eq_in_repeat() {
        let code = "PROGRAM IS
a, b
IN
REPEAT
    WRITE 1;
    READ a;
    READ b;
UNTIL a != b;
END";

        tester(code, "not_eq_in_repeat_1.txt", &[1, 1, 2, 2, 3, 3, 4, 5], &[1]);
        tester(code, "not_eq_in_repeat_2.txt", &[1, 2, 3, 1, 2, 2], &[1, 1, 1]);
    }

    #[test]
    fn lt_in_repeat() {
        let code = "PROGRAM IS
a, b
IN
REPEAT
    WRITE 1;
    READ a;
    READ b;
UNTIL a < b;
END";

        tester(code, "lt_in_repeat_1.txt", &[1, 1, 2, 2, 3, 3, 5, 4], &[1]);
        tester(code, "lt_in_repeat_2.txt", &[1, 2, 3, 6, 3, 1], &[1, 1, 1]);
    }

    #[test]
    fn le_in_repeat() {
        let code = "PROGRAM IS
a, b
IN
REPEAT
    WRITE 1;
    READ a;
    READ b;
UNTIL a <= b;
END";

        tester(code, "le_in_repeat_1.txt", &[1, 1, 2, 2, 3, 3, 5, 4], &[1, 1, 1, 1]);
        tester(code, "le_in_repeat_2.txt", &[1, 2, 3, 6, 3, 1], &[1, 1, 1]);
    }

    #[test]
    fn gr_in_repeat() {
        let code = "PROGRAM IS
a, b
IN
REPEAT
    WRITE 1;
    READ a;
    READ b;
UNTIL a > b;
END";

        tester(code, "gr_in_repeat_1.txt", &[1, 1, 2, 2, 3, 3, 4, 5], &[1]);
        tester(code, "gr_in_repeat_2.txt", &[2, 1, 6, 3, 1, 3], &[1, 1, 1]);
    }

    #[test]
    fn ge_in_repeat() {
        let code = "PROGRAM IS
a, b
IN
REPEAT
    WRITE 1;
    READ a;
    READ b;
UNTIL a >= b;
END";

        tester(code, "ge_in_repeat_1.txt", &[1, 1, 2, 2, 3, 3, 4, 5], &[1, 1, 1, 1]);
        tester(code, "ge_in_repeat_2.txt", &[2, 1, 6, 3, 1, 3], &[1, 1, 1]);
    }

    #[test]
    fn example_2() {
        file_tester("../examples/example2.imp", "example_2.txt", &[0, 1], &[46368, 28657])
    }

    #[test]
    fn example_3() {
        file_tester("../examples/example3.imp", "example_3.txt", &[1], &[121393])
    }

    #[test]
    fn example_7() {
        file_tester("../examples/example7.imp", "example_7_1.txt", &[0, 0, 0], &[31000, 40900, 2222010]);
        file_tester("../examples/example7.imp", "example_7_2.txt", &[1, 0, 2], &[31001, 40900, 2222012]);
    }

    #[test]
    fn program_1() {
        file_tester("../examples/program1.imp", "program_1.txt", &[21, 7, 21, 6], &[1]);
    }

    #[test]
    fn simple_mul() {
        let code = "PROGRAM IS
a, b
IN
READ a ;
READ b ;
a := a * b;
WRITE a;
END";
    
            tester(code, "simple_mul_1.txt", &[2, 3], &[6]);
            tester(code, "simple_mul_2.txt", &[3, 2], &[6]);
            tester(code, "simple_mul_3.txt", &[3, 0], &[0]);
            tester(code, "simple_mul_4.txt", &[0, 3], &[0]);
        }

    #[test]
    fn simple_div() {
        let code = "PROGRAM IS
a, b
IN
READ a ;
READ b ;
a := a / b;
WRITE a;
END";
    
            tester(code, "simple_div_1.txt", &[6, 3], &[2]);
            tester(code, "simple_div_2.txt", &[3, 2], &[1]);
            tester(code, "simple_div_3.txt", &[3, 0], &[0]);
            tester(code, "simple_div_4.txt", &[0, 3], &[0]);
        }
}
