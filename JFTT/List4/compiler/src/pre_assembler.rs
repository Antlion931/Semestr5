use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum AbstractVarible {
    Table(String, AbstractNumber),
    Else(AbstractNumber),
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum AbstractNumber {
    Accumulator,
    Temp(String),
    Var(String),
    Const(u64),
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum PreAssembler {
    READ,
    WRITE,
    ADD(AbstractVarible),
    SUB(AbstractVarible),
    GET(AbstractVarible),
    PUT(AbstractVarible),
    RST(AbstractVarible),
    INC(AbstractVarible),
    DEC(AbstractVarible),
    SHL(AbstractVarible),
    SHR(AbstractVarible),
}

pub enum Jumps {
    JUMP(Rc<Block>),
    JPOS { t: Rc<Block>, f: Rc<Block> },
    JZERO { t: Rc<Block>, f: Rc<Block> },
    HALT,
}

pub struct Block {
    pub pre_assembler: Vec<PreAssembler>,
    pub jumps: Jumps,
    pub memory: HashMap<AbstractVarible, u64>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            pre_assembler: Vec::new(),
            jumps: Jumps::HALT,
            memory: HashMap::new(),
        }
    }
}
