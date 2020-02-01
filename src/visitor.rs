use crate::token::Token;

pub trait Visitor {
    fn visit_token(&mut self, node: &Token);
}

pub struct Compiler;

impl Visitor for Compiler {
    fn visit_token(&mut self, node: &Token) {
        match node {
            &Token::Add => {}
            &Token::Sub => {}
            &Token::Multiple => {}
            &Token::Div => {}
            &Token::Int { value } => {},
            &Token::Expression { ref left, ref op, ref right } => {},
            &Token::Program { ref expression } => {}
        }
    }
}
