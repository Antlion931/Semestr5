use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum AbstractVarible {
    Table(String, AbstractNumber),
    Else(AbstractNumber),
}

impl AbstractVarible {
    pub fn deref(&self) -> Option<&Self> {
        match self {
            AbstractVarible::Else(AbstractNumber::Pointer(x)) => Some(x.as_ref()),
            _ => None,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum ProcType {
    Procedure(String),
    Main,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum AbstractNumber {
    Accumulator,
    ProcedureReturn(ProcType),
    Pointer(Box<AbstractVarible>),
    Var(ProcType, String),
    Const(u64),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum PreAssembler {
    READ,
    WRITE,
    ADD(AbstractVarible),
    SUB(AbstractVarible),
    GET(AbstractVarible),
    PUT(AbstractVarible),
    INC(AbstractVarible),
    MOVE(char),
    MUL,
    DIV,
    MOD,
    LABEL(Label),
    JUMP(Label),
    JPOS(Label),
    JZERO(Label),
    JUMPR(AbstractVarible),
    STRK(AbstractVarible),
    HALT,
}

type Label = u64;

pub struct Block {
    pub pre_assembler: Vec<PreAssembler>,
    pub memory: HashMap<AbstractVarible, u64>,
    pub mul_label: Option<Label>,
    pub div_label: Option<Label>,
    pub mod_label: Option<Label>,
}

impl Block {
    pub fn new() -> Self {
        Self {
            pre_assembler: Vec::new(),
            memory: HashMap::new(),
            mul_label: None,
            div_label: None,
            mod_label: None,
        }
    }
}
