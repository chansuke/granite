#[derive(Debug, PartialEq)]
pub enum Token {
    Add,
    Sub,
    Multiple,
    Div,
    Int {
        value: i64,
    },
    Expression {
        left: Box<Token>,
        op: Box<Token>,
        right: Box<Token>,
    },
    Program {
        expressions: Vec<Token>,
    },
}
