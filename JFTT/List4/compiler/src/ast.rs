use std::hash::Hash;

pub trait DiagnosticInfo {
    fn get_name(&self) -> &str;
    fn get_start(&self) -> usize;
    fn get_end(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct AST {
    pub procedurs: Vec<Procedure>,
    pub main: Main,
    pub start: usize,
    pub end: usize,
}

impl AST {
    pub fn new(procedurs: Vec<Procedure>, main: Main, start: usize, end: usize) -> Self {
        Self {
            procedurs,
            main,
            start,
            end,
        }
    }
}

impl DiagnosticInfo for AST {
    fn get_name(&self) -> &str {
        "AST"
    }

    fn get_start(&self) -> usize {
        self.start
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Clone)]
pub struct Procedure {
    pub proc_head: ProcHead,
    pub declarations: Vec<Declaration>,
    pub commands: Vec<Command>,
    pub start: usize,
    pub end: usize,
}

impl Procedure {
    pub fn new(
        proc_head: ProcHead,
        declarations: Vec<Declaration>,
        commands: Vec<Command>,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            proc_head,
            declarations,
            commands,
            start,
            end,
        }
    }
}

impl DiagnosticInfo for Procedure {
    fn get_name(&self) -> &str {
        self.proc_head.name.as_str()
    }

    fn get_start(&self) -> usize {
        self.start
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Clone)]
pub struct Main {
    pub declarations: Vec<Declaration>,
    pub commands: Vec<Command>,
    pub start: usize,
    pub end: usize,
}

impl Main {
    pub fn new(
        declarations: Vec<Declaration>,
        commands: Vec<Command>,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            declarations,
            commands,
            start,
            end,
        }
    }
}

impl DiagnosticInfo for Main {
    fn get_name(&self) -> &str {
        "main"
    }

    fn get_start(&self) -> usize {
        self.start
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Clone)]
pub struct ProcHead {
    pub name: String,
    pub params: Vec<ArgDecl>,
    pub start: usize,
    pub end: usize,
}

impl ProcHead {
    pub fn new(name: String, params: Vec<ArgDecl>, start: usize, end: usize) -> Self {
        Self {
            name,
            params,
            start,
            end,
        }
    }
}

impl DiagnosticInfo for ProcHead {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_start(&self) -> usize {
        self.start
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Clone)]
pub struct ArgDecl {
    pub name: String,
    pub arg_type: ArgType,
    pub start: usize,
    pub end: usize,
}

impl ArgDecl {
    pub fn new(name: String, arg_type: ArgType, start: usize, end: usize) -> Self {
        Self {
            name,
            arg_type,
            start,
            end,
        }
    }
}

impl DiagnosticInfo for ArgDecl {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_start(&self) -> usize {
        self.start
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Clone)]
pub enum ArgType {
    Number,
    Table,
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub name: String,
    pub decl_type: DeclType,
    pub start: usize,
    pub end: usize,
}

impl Declaration {
    pub fn new(name: String, decl_type: DeclType, start: usize, end: usize) -> Self {
        Self {
            name,
            decl_type,
            start,
            end,
        }
    }
}

impl DiagnosticInfo for Declaration {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_start(&self) -> usize {
        self.start
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Clone)]
pub enum DeclType {
    Number,
    Table(u64),
}

#[derive(Debug, Clone)]
pub enum Command {
    Assign(Assign),
    If(If),
    While(While),
    Repeat(Repeat),
    Call(Call),
    Read(Read),
    Write(Write),
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub identifier: Identifier,
    pub expression: Expression,
    pub start: usize,
    pub end: usize,
}

impl Assign {
    pub fn new(identifier: Identifier, expression: Expression, start: usize, end: usize) -> Self {
        Self {
            identifier,
            expression,
            start,
            end,
        }
    }
}

impl DiagnosticInfo for Assign {
    fn get_name(&self) -> &str {
        self.identifier.name.as_str()
    }

    fn get_start(&self) -> usize {
        self.start
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Clone)]
pub struct Identifier {
    pub name: String,
    pub indentifier_type: IdentifierType,
    pub start: usize,
    pub end: usize,
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.indentifier_type == other.indentifier_type
    }
}

impl Eq for Identifier {}

impl Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.indentifier_type.hash(state);
    }
}

impl Identifier {
    pub fn new(name: String, indentifier_type: IdentifierType, start: usize, end: usize) -> Self {
        Self {
            name,
            indentifier_type,
            start,
            end,
        }
    }
}

impl DiagnosticInfo for Identifier {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_start(&self) -> usize {
        self.start
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Clone)]
pub enum IdentifierType {
    Number,
    TableWithNumber(u64),
    TableWithIdentifier(String),
}

impl PartialEq for IdentifierType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (IdentifierType::Number, IdentifierType::Number) => true,
            (IdentifierType::TableWithNumber(_), IdentifierType::TableWithNumber(_)) => true,
            (IdentifierType::TableWithIdentifier(_), IdentifierType::TableWithIdentifier(_)) => {
                true
            }
            _ => false,
        }
    }
}

impl Eq for IdentifierType {}

impl Hash for IdentifierType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            IdentifierType::Number => 0.hash(state),
            IdentifierType::TableWithNumber(_) => 1.hash(state),
            IdentifierType::TableWithIdentifier(_) => 2.hash(state),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(Value),
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
    Mod(Value, Value),
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(u64),
    Identifier(Identifier),
}

#[derive(Debug, Clone)]
pub enum Condition {
    Equal(Value, Value),
    NotEqual(Value, Value),
    Less(Value, Value),
    LessEqual(Value, Value),
    Greater(Value, Value),
    GreaterEqual(Value, Value),
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Condition,
    pub then_commands: Vec<Command>,
    pub else_commands: Vec<Command>,
    pub start: usize,
    pub end: usize,
}

impl If {
    pub fn new(
        condition: Condition,
        then_commands: Vec<Command>,
        else_commands: Vec<Command>,
        start: usize,
        end: usize,
    ) -> Self {
        Self {
            condition,
            then_commands,
            else_commands,
            start,
            end,
        }
    }
}

impl DiagnosticInfo for If {
    fn get_name(&self) -> &str {
        "if"
    }

    fn get_start(&self) -> usize {
        self.start
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Condition,
    pub commands: Vec<Command>,
    pub start: usize,
    pub end: usize,
}

impl While {
    pub fn new(condition: Condition, commands: Vec<Command>, start: usize, end: usize) -> Self {
        Self {
            condition,
            commands,
            start,
            end,
        }
    }
}

impl DiagnosticInfo for While {
    fn get_name(&self) -> &str {
        "while"
    }

    fn get_start(&self) -> usize {
        self.start
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Clone)]
pub struct Repeat {
    pub commands: Vec<Command>,
    pub condition: Condition,
    pub start: usize,
    pub end: usize,
}

impl Repeat {
    pub fn new(commands: Vec<Command>, condition: Condition, start: usize, end: usize) -> Self {
        Self {
            commands,
            condition,
            start,
            end,
        }
    }
}

impl DiagnosticInfo for Repeat {
    fn get_name(&self) -> &str {
        "repeat"
    }

    fn get_start(&self) -> usize {
        self.start
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Clone)]
pub struct Call {
    pub name: String,
    pub args: Vec<Arg>,
    pub start: usize,
    pub end: usize,
}

impl Call {
    pub fn new(name: String, args: Vec<Arg>, start: usize, end: usize) -> Self {
        Self {
            name,
            args,
            start,
            end,
        }
    }
}

impl DiagnosticInfo for Call {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_start(&self) -> usize {
        self.start
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Clone)]
pub struct Arg {
    pub name: String,
    pub start: usize,
    pub end: usize,
}

impl Arg {
    pub fn new(name: String, start: usize, end: usize) -> Self {
        Self { name, start, end }
    }
}

impl DiagnosticInfo for Arg {
    fn get_name(&self) -> &str {
        self.name.as_str()
    }

    fn get_start(&self) -> usize {
        self.start
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Clone)]
pub struct Read {
    pub identifier: Identifier,
    pub start: usize,
    pub end: usize,
}

impl Read {
    pub fn new(identifier: Identifier, start: usize, end: usize) -> Self {
        Self {
            identifier,
            start,
            end,
        }
    }
}

impl DiagnosticInfo for Read {
    fn get_name(&self) -> &str {
        self.identifier.name.as_str()
    }

    fn get_start(&self) -> usize {
        self.start
    }

    fn get_end(&self) -> usize {
        self.end
    }
}

#[derive(Debug, Clone)]
pub struct Write {
    pub value: Value,
    pub start: usize,
    pub end: usize,
}

impl Write {
    pub fn new(value: Value, start: usize, end: usize) -> Self {
        Self { value, start, end }
    }
}

impl DiagnosticInfo for Write {
    fn get_name(&self) -> &str {
        "write"
    }

    fn get_start(&self) -> usize {
        self.start
    }

    fn get_end(&self) -> usize {
        self.end
    }
}
