use std::iter::Peekable;
use std::str::Chars;
use tokenizer::Token;
use tokenizer::Keyword;

pub fn consume_while<F>(it: &mut Peekable<Chars>, pred: F) -> Vec<char>
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

pub fn consume_token(it: &mut Peekable<Chars>, token_vec: &mut Vec<Token>, token: Token) {
  it.next();
  token_vec.push(token);
}

pub fn consume_number(it: &mut Peekable<Chars>, token_vec: &mut Vec<Token>, errors: &mut Vec<String>, line: i32) {
  let num: String = consume_while(it, |a| a.is_numeric())
    .into_iter()
    .collect();

  if it.peek().unwrap() == &'.' {
    it.next();
    let decimal: String = consume_while(it, |a| a.is_numeric())
      .into_iter()
      .collect();

    let float = [num.as_ref(), ".", decimal.as_ref()].join("");

    if it.peek().unwrap().is_alphabetic() {
      errors.push(format!("Unexpected character in number on Line {}", line));
    } else {
      token_vec.push(Token::Float(float.parse::<f32>().unwrap()));
    }
  } else {
    if it.peek().unwrap().is_alphabetic() {
      errors.push(format!("Unexpected character in number on Line {}", line));
    } else {
      token_vec.push(Token::Integer(num.parse::<i32>().unwrap()));
    }
  }
}

pub fn consume_keyword(it: &mut Peekable<Chars>, token_vec: &mut Vec<Token>) {
  let chars: String = consume_while(it, |a| a.is_alphanumeric() || a == '_')
    .into_iter()
    .collect();

  match chars.as_ref() {
    "def" => token_vec.push(Token::Keyword(Keyword::Def)),
    _ => token_vec.push(Token::Variable(chars))
  }
}

pub fn consume_string(it: &mut Peekable<Chars>, token_vec: &mut Vec<Token>) {
  it.next();
  let chars: String = consume_while(it, |a| a != '\'')
    .into_iter()
    .collect();
  it.next();

  token_vec.push(Token::String(chars));
}

pub fn consume_comment(it: &mut Peekable<Chars>, token_vec: &mut Vec<Token>) {
  it.next();

  let comment: String = consume_while(it, |a| a != '\n')
    .into_iter()
    .collect();

  token_vec.push(Token::Comment(comment));
}