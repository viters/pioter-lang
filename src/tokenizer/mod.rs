use tokenizer::keyword::*;
use tokenizer::symbol::*;
use tokenizer::consumers::*;

pub mod keyword;
pub mod symbol;
pub mod consumers;

#[derive(Debug, PartialEq)]
pub enum Token {
  Float(f32),
  Integer(i32),
  Operator(Symbol),
  Keyword(Keyword),
  Variable(String),
  String(String),
}

pub trait Tokenizer {
  fn tokenize(&self) -> Result<Vec<Token>, Vec<String>>;
}

impl Tokenizer for String {
  fn tokenize(&self) -> Result<Vec<Token>, Vec<String>> {
    let mut it = self.chars().peekable();
    let mut tokens: Vec<Token> = vec![];
    let mut line = 1;
    let mut errors: Vec<String> = vec![];

    loop {
      match it.peek() {
        Some(&ch) => match ch {
          '0' ... '9' => consume_number(&mut it, &mut tokens, &mut errors, line ),
          '+' => consume_token(&mut it, &mut tokens, Token::Operator(Symbol::Plus)),
          '-' => {
            it.next();
            if it.peek().unwrap() == &'>' {
              it.next();
              tokens.push(Token::Operator(Symbol::Arrow));
            } else {
              tokens.push(Token::Operator(Symbol::Minus));
            }
          }
          '*' => consume_token(&mut it, &mut tokens, Token::Operator(Symbol::Multiply)),
          '/' => consume_token(&mut it, &mut tokens, Token::Operator(Symbol::Divide)),
          '(' => consume_token(&mut it, &mut tokens, Token::Operator(Symbol::LParen)),
          ')' => consume_token(&mut it, &mut tokens, Token::Operator(Symbol::RParen)),
          '[' => consume_token(&mut it, &mut tokens, Token::Operator(Symbol::LSquareBracket)),
          ']' => consume_token(&mut it, &mut tokens, Token::Operator(Symbol::RSquareBracket)),
          ' ' => consume_token(&mut it, &mut tokens, Token::Operator(Symbol::Space)),
          '\r' => consume_token(&mut it, &mut tokens, Token::Operator(Symbol::CR)),
          '\n' => {
            consume_token(&mut it, &mut tokens, Token::Operator(Symbol::LB));
            line = line + 1;
          }
          '\'' => consume_string(&mut it, &mut tokens),
          'A' ... 'Z' | 'a' ... 'z' | '_' => consume_keyword(&mut it, &mut tokens),
          _ => {
            let ch = it.next().unwrap();
            errors.push(format!("Unknown character \"{}\" on Line {}", ch, line))
          }
        },
        None => break
      }
    }

    if errors.len() > 0 {
      Err(errors)
    } else {
      Ok(tokens)
    }
  }
}

