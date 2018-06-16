use tokenizer::consumers::*;
use tokenizer::symbol::Symbol;

pub mod keyword;
pub mod symbol;
pub mod consumers;

#[derive(Debug, PartialEq)]
pub enum Token {
  Float(f32),
  Integer(i32),
  Symbol(symbol::Symbol),
  Keyword(keyword::Keyword),
  Constant(String),
  String(String),
  Comment(String),
}

pub fn tokenize(contents: &String) -> Result<Vec<Token>, Vec<String>> {
  let mut it = contents.chars().peekable();
  let mut tokens: Vec<Token> = vec![];
  let mut line = 1;
  let mut errors: Vec<String> = vec![];

  loop {
    match it.peek() {
      Some(&ch) => match ch {
        '0'...'9' => consume_number(&mut it, &mut tokens, &mut errors, line),
        '-' => {
          it.next();
          if it.peek().unwrap().to_owned() == '>' {
            it.next();
            tokens.push(Token::Symbol(Symbol::Arrow));
          } else {
            consume_constant(&mut it, &mut tokens);
          }
        }
        '(' => consume_token(&mut it, &mut tokens, Token::Symbol(Symbol::LParen)),
        ')' => consume_token(&mut it, &mut tokens, Token::Symbol(Symbol::RParen)),
        '[' => consume_token(&mut it, &mut tokens, Token::Symbol(Symbol::LSquareBracket)),
        ']' => consume_token(&mut it, &mut tokens, Token::Symbol(Symbol::RSquareBracket)),
        '{' => consume_token(&mut it, &mut tokens, Token::Symbol(Symbol::LCurlyBracket)),
        '}' => consume_token(&mut it, &mut tokens, Token::Symbol(Symbol::RCurlyBracket)),
        '#' => consume_comment(&mut it, &mut tokens),
        ',' => consume_token(&mut it, &mut tokens, Token::Symbol(Symbol::Comma)),
        ' ' => consume_token(&mut it, &mut tokens, Token::Symbol(Symbol::Space)),
        '\r' => consume_token(&mut it, &mut tokens, Token::Symbol(Symbol::CR)),
        '\n' => {
          consume_token(&mut it, &mut tokens, Token::Symbol(Symbol::LB));
          line = line + 1;
        }
        '\'' => consume_string(&mut it, &mut tokens),
        'A'...'Z' | 'a'...'z' | '+' | '*' | '/' => consume_constant(&mut it, &mut tokens),
        '_' => consume_token(&mut it, &mut tokens, Token::Symbol(Symbol::Underscore)),
        ':' => consume_token(&mut it, &mut tokens, Token::Symbol(Symbol::Colon)),
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
