use calculator::*;
use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Debug)]
pub struct ParserResult {
    pub rpn: String,
    pub gf: GFResult,
}

impl ParserResult {
    pub fn new(rpn: String, gf: GFResult ) -> Self {
        Self { rpn, gf}
    }

    pub fn ignore() -> Option<Self> {
        None
    }
}

impl Display for ParserResult {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match self.gf {
            Ok(x) => write!(fmt, "RPN: {}\n= {}", self.rpn, x),
            Err(e) => write!(fmt, "RPN: {}\n= {}", self.rpn, e),
        }
    }
}

pub struct BinaryOperation {
    pub f: Box<dyn Fn(GFResult, GFResult) -> GFResult>,
    pub str: &'static str,
}

impl BinaryOperation {
    pub fn new(f: impl Fn(GFResult, GFResult) -> GFResult + 'static, str: &'static str) -> Self {
        Self {
            f: Box::new(f),
            str,
        }
    }
}

pub struct UnaryOperation {
    pub f: Box<dyn Fn(GFResult) -> GFResult>,
    pub str: &'static str,
}

impl UnaryOperation {
    pub fn new(f: impl Fn(GFResult) -> GFResult + 'static, str: &'static str) -> Self {
        Self {
            f: Box::new(f),
            str,
        }
    }
}
