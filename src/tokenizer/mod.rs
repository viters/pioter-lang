use std::iter::Peekable;
use std::str::Chars;
use tokenizer::keyword::*;
use tokenizer::symbol::*;

pub mod keyword;
pub mod symbol;

#[derive(Debug, PartialEq)]
pub enum Token {
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
          '0' ... '9' => {
            let num: String = consume_while(&mut it, |a| a.is_numeric())
              .into_iter()
              .collect();
            if it.peek().unwrap().is_alphabetic() {
              errors.push(format!("Unexpected character in number on Line {}", line));
            } else {
              tokens.push(Token::Integer(num.parse::<i32>().unwrap()));
            }
          }
          '+' => {
            it.next();
            tokens.push(Token::Operator(Symbol::Plus));
          }
          '-' => {
            it.next();

            match it.peek() {
              Some(&ch) => match ch {
                '>' => {
                  it.next();
                  tokens.push(Token::Operator(Symbol::Arrow));
                }
                _ => {
                  tokens.push(Token::Operator(Symbol::Minus));
                }
              },
              None => {
                tokens.push(Token::Operator(Symbol::Minus));
                break;
              }
            }
          }
          '*' => {
            it.next();
            tokens.push(Token::Operator(Symbol::Multiply));
          }
          '/' => {
            it.next();
            tokens.push(Token::Operator(Symbol::Divide));
          }
          '(' => {
            it.next();
            tokens.push(Token::Operator(Symbol::LParen));
          }
          ')' => {
            it.next();
            tokens.push(Token::Operator(Symbol::RParen));
          }
          '[' => {
            it.next();
            tokens.push(Token::Operator(Symbol::LSquareBracket));
          }
          ']' => {
            it.next();
            tokens.push(Token::Operator(Symbol::RSquareBracket));
          }
          ' ' => {
            it.next();
            tokens.push(Token::Operator(Symbol::Space));
          }
          '\r' => {
            it.next();
            tokens.push(Token::Operator(Symbol::CR));
          }
          '\n' => {
            it.next();
            tokens.push(Token::Operator(Symbol::LB));
            line = line + 1;
          }
          '\'' => {
            it.next();
            let chars: String = consume_while(&mut it, |a| a != '\'')
              .into_iter()
              .collect();
            it.next();

            tokens.push(Token::String(chars));
          }
          'A' ... 'Z' | 'a' ... 'z' | '_' => {
            let chars: String = consume_while(&mut it, |a| a.is_alphanumeric() || a == '_')
              .into_iter()
              .collect();

            match chars.as_ref() {
              "def" => tokens.push(Token::Keyword(Keyword::Def)),
              _ => tokens.push(Token::Variable(chars))
            }
          }
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

fn consume_while<F>(it: &mut Peekable<Chars>, pred: F) -> Vec<char>
  where F: Fn(char) -> bool {
  let mut chars: Vec<char> = vec![];

  while let Some(&ch) = it.peek() {
    if pred(ch) {
      it.next().unwrap();
      chars.push(ch);
    } else {
      break;
    }
  }

  chars
}
