use crate::token::Token;

pub trait Visitor {
    fn visit_token(&mut self, node: &Token);
}

pub struct Compiler;

impl Visitor for Compiler {
    fn visit_token(&mut self, node: &Token) {
        match node {
            &Token::Add => {
                println!("Add operator!");
            }
            &Token::Sub => {}
            &Token::Multiple => {}
            &Token::Div => {}
            &Token::Int { value } => {
                println!("Integer {:#?}: ", value);
            }
            &Token::Expression {
                ref left,
                ref op,
                ref right,
            } => {
                println!("Visiting an expression");
                self.visit_token(left);
                self.visit_token(op);
                self.visit_token(right);
                println!("Done visiting expression");
            }
            &Token::Program { ref expressions } => {
                println!("Visiting program");
                for expression in expressions {
                    self.visit_token(expression);
                }
                println!("Done visiting program");
            }
        }
    }
}

mod tests {
    use super::*;
    use crate::parser::program;
    use nom::types::CompleteStr;

    fn generate_test_program() -> Token {
        let source = CompleteStr("1+2");
        let (_, tree) = program(source).unwrap();
        tree
    }

    #[test]
    fn test_visit_addition_token() {
        let mut compiler = Compiler {};
        let test_program = generate_test_program();
        compiler.visit_token(&test_program);
    }
}
