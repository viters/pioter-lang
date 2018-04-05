use tokenizer::keyword::*;
use tokenizer::symbol::*;
use tokenizer::utils::*;

pub mod keyword;
pub mod symbol;
pub mod utils;

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
          '+' => consume_token(&mut it, &mut tokens, Token::Operator(Symbol::Plus)),
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

