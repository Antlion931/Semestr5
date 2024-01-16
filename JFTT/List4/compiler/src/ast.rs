#[derive(Debug)]
pub struct AST {
    pub procedurs: Vec<Procedure>,
    pub main: Main,
}

impl AST {
    pub fn new(procedurs: Vec<Procedure>, main: Main) -> Self {
        Self { procedurs, main }
    }
}

#[derive(Debug)]
pub struct Procedure {
    pub proc_head: ProcHead,
    pub declarations: Vec<Declaration>,
    pub commands: Vec<Command>,
}

impl Procedure {
    pub fn new(proc_head: ProcHead, declarations: Vec<Declaration>, commands: Vec<Command>) -> Self {
        Self {
            proc_head,
            declarations,
            commands,
        }
    }
}

#[derive(Debug)]
pub struct Main {
    pub declarations: Vec<Declaration>,
    pub commands: Vec<Command>,
}

impl Main {
    pub fn new(declarations: Vec<Declaration>, commands: Vec<Command>) -> Self {
        Self {
            declarations,
            commands,
        }
    }
}

#[derive(Debug)]
pub struct ProcHead {
    pub name: String,
    pub params: Vec<ArgDecl>,
}

impl ProcHead {
    pub fn new(name: String, params: Vec<ArgDecl>) -> Self {
        Self { name, params }
    }
}

#[derive(Debug)]
pub struct ArgDecl {
    pub name: String,
    pub arg_type: ArgType,
}

impl ArgDecl {
    pub fn new(name: String, arg_type: ArgType) -> Self {
        Self { name, arg_type }
    }
}

#[derive(Debug)]
pub enum ArgType {
    Number,
    Table,
}

#[derive(Debug)]
pub struct Declaration {
    pub name: String,
    pub decl_type: DeclType,
}

impl Declaration {
    pub fn new(name: String, decl_type: DeclType) -> Self {
        Self { name, decl_type }
    }
}

#[derive(Debug)]
pub enum DeclType {
    Number,
    Table(u64),
}

#[derive(Debug)]
pub enum Command {
    Assign(Assign),
    If(If),
    While(While),
    Repeat(Repeat),
    Call(Call),
    Read(Read),
    Write(Write),
}

#[derive(Debug)]
pub struct Assign {
    pub identifier: Identifier,
    pub expression: Expression,
}

impl Assign {
    pub fn new(identifier: Identifier, expression: Expression) -> Self {
        Self {
            identifier,
            expression,
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    pub name: String,
    pub indentifier_type: IdentifierType,
}

impl Identifier {
    pub fn new(name: String, indentifier_type: IdentifierType) -> Self {
        Self {
            name,
            indentifier_type,
        }
    }
}

#[derive(Debug)]
pub enum IdentifierType {
    Number,
    TableWithNumber(u64),
    TableWithIdentifier(String),
}

#[derive(Debug)]
pub enum Expression {
    Number(Value),
    Add(Value, Value),
    Sub(Value, Value),
    Mul(Value, Value),
    Div(Value, Value),
    Mod(Value, Value),
}

#[derive(Debug)]
pub enum Value {
    Number(u64),
    Identifier(Identifier),
}

#[derive(Debug)]
pub enum Condition {
    Equal(Value, Value),
    NotEqual(Value, Value),
    Less(Value, Value),
    LessEqual(Value, Value),
    Greater(Value, Value),
    GreaterEqual(Value, Value),
}

#[derive(Debug)]
pub struct If {
    pub condition: Condition,
    pub then_commands: Vec<Command>,
    pub else_commands: Vec<Command>,
}

impl If {
    pub fn new(
        condition: Condition,
        then_commands: Vec<Command>,
        else_commands: Vec<Command>,
    ) -> Self {
        Self {
            condition,
            then_commands,
            else_commands,
        }
    }
}

#[derive(Debug)]
pub struct While {
    pub condition: Condition,
    pub commands: Vec<Command>,
}

impl While {
    pub fn new(condition: Condition, commands: Vec<Command>) -> Self {
        Self { condition, commands }
    }
}

#[derive(Debug)]
pub struct Repeat {
    pub commands: Vec<Command>,
    pub condition: Condition,
}

impl Repeat {
    pub fn new(commands: Vec<Command>, condition: Condition) -> Self {
        Self { commands, condition }
    }
}

#[derive(Debug)]
pub struct Call {
    pub name: String,
    pub args: Vec<Arg>,
}

impl Call {
    pub fn new(name: String, args: Vec<Arg>) -> Self {
        Self { name, args }
    }
}

#[derive(Debug)]
pub struct Arg {
    pub name: String,
}

impl Arg {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
pub struct Read {
    pub identifier: Identifier,
}

impl Read {
    pub fn new(identifier: Identifier) -> Self {
        Self { identifier }
    }
}

#[derive(Debug)]
pub struct Write {
    pub value: Value,
}

impl Write {
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}
