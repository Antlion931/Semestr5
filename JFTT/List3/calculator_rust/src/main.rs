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

                if combined_lines_buffer.len() > 0 {
                    let result = parser::ExprParser::new().parse(&combined_lines_buffer);
                    match result {
                        
                        Ok(Some(result)) => println!("{}", result),
                        Err(_) => println!("Error with syntax"),
                        _ => {},
                    }
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
        let r = parser::ExprParser::new().parse(line).unwrap().unwrap();
        assert_eq!(r.gf, GF::try_new(expected_value, 1234577));
    }

    #[test]
    fn test_one() {
        test("2+3*(4-5)", 1234576);
    }

    #[test]
    fn test_two() {
        test("2^100", 295422);
    }

    #[test]
    fn test_three() {
        let r = parser::ExprParser::new().parse("# ala ma kota").unwrap();
        assert!(r.is_none());
    }

    #[test]
    fn test_four() {
        test("2-3-2", 1234574);
    }

    #[test]
    fn test_five() {
        test("269164/123456", 567890);
    }

    #[test]
    fn test_six() {
        test("-2--1", 1234576);
    }

    #[test]
    fn test_seven() {
        test("1/-580978", 123456);
    }

    #[test]
    fn test_eight() {
        test("123456789", 1233666);
    }

    #[test]
    fn test_ten() {
        test("-1234567", 10);
    }

    #[test]
    fn test_eleven() {
        let r = parser::ExprParser::new().parse("2+3*(4-5");
        assert!(r.is_err());
    }

    #[test]
    fn test_twelve() {
        test("2^123", 594706);
    }

    #[test]
    fn test_thirteen() {
        test("2^-2", 925933);
    }

    #[test]
    fn test_fourteen() {
        test("(2^-1)*(2^1)", 1);
    }

    #[test]
    fn test_fifteen() {
        test("(2^-2)*(2^2)", 1);
    }

    #[test]
    fn test_sixteen() {
        test("-(17*18)", 1234271);
    }

    #[test]
    fn test_seventeen() {
        test("2^(3-4) * 2", 1);
    }

    #[test]
    fn test_eighteen() {
        test("2^999999999", 185209);
    }

    #[test]
    fn test_nineteen() {
        let r = parser::ExprParser::new().parse("2 * 3 - 17 # komentarz");
        assert!(r.is_err());
    }

    #[test]
    fn test_twenty() {
        let r = parser::ExprParser::new().parse("1/0").unwrap().unwrap();
        assert!(r.gf.is_err());
    }

    #[test]
    fn test_twenty_one() {
        let r = parser::ExprParser::new().parse("2^(1/2)").unwrap().unwrap();
        assert!(r.gf.is_err());
    }

    #[test]
    fn test_twenty_two() {
        test("(2^(1/3)) ^ 3", 2);
    }

    #[test]
    fn test_twenty_three() {
        test("2 + 3+ 4 +5", 14);
    }

    #[test]
    fn test_twenty_four() {
        test("---1", 1234576);
    }

    #[test]
    fn test_twenty_five() {
        test("----1", 1);
    }

    #[test]
    fn test_twenty_six() {
        test("---(1)", 1234576);
    }

    #[test]
    fn test_twenty_seven() {
        test("----(1)", 1);
    }

    #[test]
    fn test_minus_2_squared() {
        test("-2^2", 1234573);
    }
}
