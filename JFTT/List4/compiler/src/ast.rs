pub struct AST {
    procedurs: Vec<Procedure>,
    main: Main,
}

pub struct Procedure {
    name: String,
    arguments: Vec<Var>,
    varbiles: Vec<Var>,
    commands: Vec<Command>,
}

pub struct Main {
    varbiles: Vec<Var>,
    commands: Vec<Command>,
}

pub struct Var {
    name: String,
    var_type: VarType,
}

pub enum VarType {
    Number,
    Table,
}

pub enum Command {
    Assignemt(Assignment),
    If(If),
    While(While),
    Repeat(Repeat),
    Call(Call),
    Read(Read),
    Write(Write),
}

pub struct Assignment {
    var: Var,
    expr: Expr,
}

pub enum Expr {
    Var(Var),
    Add(Var, Var),
    Sub(Var, Var),
    Mul(Var, Var),
    Div(Var, Var),
    Mod(Var, Var),
}

pub struct If {
    condition: Condition,
    commands: Vec<Command>,
    else_commands: Vec<Command>,
}

pub enum Condition {
    Equal(Var, Var),
    Less(Var, Var),
}

