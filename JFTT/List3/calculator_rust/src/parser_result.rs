use calculator::*;
use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Debug)]
pub struct ParserResult {
    pub rpn: String,
    pub rpn_error: bool,
    pub ignore: bool,
    pub value: GF,
}

impl ParserResult {
    pub fn new(rpn: String, value: GF) -> Self {
        Self { rpn, value, rpn_error: false, ignore: false}
    }

    pub fn ignore() -> Self {
        Self { rpn: "".to_string(), rpn_error: false, ignore: true, value: GF::new(0) }
    }

    pub fn broken(mut self) -> Self {
        self.rpn_error = true;
        self
    }
}

impl From<GF> for ParserResult {
    fn from(value: GF) -> Self {
        Self::new(value.to_string(), value)
    }
}

impl Display for ParserResult {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        if self.ignore {
            return Ok(());
        }

        if self.rpn_error {
            write!(fmt, "RPN: Error\n= {}", self.value)
        } else {
            write!(fmt, "RPN: {}\n= {}", self.rpn, self.value)
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
    Pow,
    Neg,
}

impl Display for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        use self::Opcode::*;
        match *self {
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            Pow => write!(fmt, "^"),
            Neg => write!(fmt, "~"),
        }
    }
}
