use nom::digit;
use nom::types::CompleteStr;

use crate::tokens::Token;

named!(pub expression<CompleteStr, Token>,
  ws!(
      do_parse!(
          left: operand >>
          op: operator >>
          right: operand >>
          (
              Token::Expression{
                  left: Box::new(left),
                  right: Box::new(right),
                  op: Box::new(op)
              }
          )
      )
  )
);

named!(pub operand<CompleteStr, Token>,
  ws!(
      alt!(
          integer
      )
  )
);

named!(integer<CompleteStr, Token>,
  ws!(
    do_parse!(
      sign: opt!(tag!("-")) >>
      reg_num: digit >>
      (
        {
          let mut tmp = String::new();
          if sign.is_some() {
            tmp.push_str("-");
          }
          tmp.push_str(&reg_num.to_string());
          let int = tmp.parse::<i64>().unwrap();
          Token::Int{ value: int }
        }
      )
    )
  )
);

named!(pub operatpr<CompleteStr, Token>,
  ws!(
    do_parse!(
      token: alt!(
        tag!("+") |
        tag!("-") |
        tag!("*") |
        tag!("/")
    ) >>
      (
        {
          match token {
            CompleteStr("+") => Token::Add,
            CompleteStr("-") => Token::Sub,
            CompleteStr("*") => Token::Multiple,
            CompleteStr("/") => Token::Div,
            CompleteStr(&_) => {unreachable!()},
          }
        }
      )
    )
  )
);

named!(program<CompleteStr, Token>,
  ws!(
      do_parse!(
          expressions: many1!(expression) >>
          (
              Token::Program {
                  expressions: expressions
              }
          )
      )
  )
);
