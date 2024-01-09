pub type Code = Vec<Statement>;

pub enum Statement {
    Print(Expression),
    Let(Binding),
}

pub struct Binding {
    name: Name,
    value: Expression,
}

pub enum Expression {
    Number(i32),
    List(Vec<Expression>),
    Function {
        input: Vec<Name>,
        body: Code,
        output: Box<Expression>,
    },
    Refer(Name),
    Call {
        function: Box<Expression>,
        input: Box<Expression>,
    },
}

pub type Name = String;
