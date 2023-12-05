//TODO:
// [ ] Dodaj ~ jako Unarna operacja odejmowania
// [ ] dodawaj testy aby szybko pracować
// [ ] Zmień nazwy na bardziej ekspresywne
// [ ] Używaj error z Lolrpop

use lalrpop_util::lalrpop_mod;
use std::io::{self, BufRead};

lalrpop_mod!(pub parser);
pub mod parser_result;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut combined_lines_buffer = String::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.ends_with('\\') {
                    combined_lines_buffer.push_str(&line[..line.len() - 1]);
                    continue;
                } else {
                    combined_lines_buffer.push_str(&line);
                }

                combined_lines_buffer = combined_lines_buffer.replace("\\", "");

                let result = parser::ExprParser::new().parse(&combined_lines_buffer);
                match result {
                    
                    Ok(result) => println!("{}", result),
                    Err(_) => println!("Error"),
                }

                combined_lines_buffer.clear();
            }
            Err(err) => println!("Error: {}", err),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use calculator::*;

    fn test(line: &str, expected_value: i64) {
        let r = parser::ExprParser::new().parse(line).unwrap();
        assert_eq!(r.value, GF::new(expected_value));
    }

    #[test]
    fn test_comments() {
        let r = parser::ExprParser::new().parse("# ala ma kota").unwrap();
        assert!(r.ignore);
    }

    #[test]
    fn test_simple_equation() {
        test("2+3*(4-5)", 1234576);
    }

    #[test]
    fn test_two_to_power_of_hundred() {
        test("2^100", 295422);
    }

    #[test]
    fn test_two_divided_by_minus_two() {
        test("2/-2", 925933);
    }

    #[test]
    fn test_minus_one() {
        test("-1", 1234576);
    }

    #[test]
    fn test_minus_minus_one() {
        test("--1", 1);
    }

    #[test]
    fn test_minus_minus_minus_one() {
        test("---1", 1234576);
    }
}
