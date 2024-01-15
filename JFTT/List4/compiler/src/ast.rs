#[derive(Debug)]
pub struct AST {
    procedurs: Vec<Procedure>,
    main: Main,
}

#[derive(Debug)]
pub struct Procedure {
    proc_head: ProcHead,
    declarations: Vec<Declaration>,
    commands: Vec<Command>,
}

#[derive(Debug)]
pub struct Main {
    declarations: Vec<Declaration>,
    commands: Vec<Command>,
}

#[derive(Debug)]
pub struct ProcHead {
    name: String,
    params: Vec<ArgDecl>,
}

#[derive(Debug)]
pub struct ArgDecl {
    name: String,
    arg_type: ArgsType,
}

#[derive(Debug)]
pub enum ArgType {
    Number,
    Table,
}

#[derive(Debug)]
pub struct Declaration {
    name: String,
    decl_type: DeclType,
}

#[derive(Debug)]
pub enum DeclType {
    Number,
    Table(u64),
}

enum Command {
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
    identifier: Identifier,
    expression: Expression,
}

#[derive(Debug)]
pub struct Identifier {
    name: String,
    indentifier_type: IdentifierType,
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
    condition: Condition,
    then_commands: Vec<Command>,
    else_commands: Vec<Command>,
}

#[derive(Debug)]
pub struct While {
    condition: Condition,
    commands: Vec<Command>,
}

#[derive(Debug)]
pub struct Repeat {
    commands: Vec<Command>,
    condition: Condition,
}

#[derive(Debug)]
pub struct Call {
    name: String,
    args: Vec<Arg>,
}

#[derive(Debug)]
pub struct Arg {
    name: String,
}

#[derive(Debug)]
pub struct Read {
    identifier: Identifier,
}

#[derive(Debug)]
pub struct Write {
    value: Value,
}
