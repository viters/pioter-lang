#[derive(Debug, PartialEq)]
pub enum Symbol {
  Plus,
  Minus,
  Multiply,
  Divide,
  LParen,
  RParen,
  LSquareBracket,
  RSquareBracket,
  LCurlyBracket,
  RCurlyBracket,
  Space,
  CR,
  LB,
  Arrow,
  Comma,
  Underscore,
  Colon
}

pub fn symbol_to_string(symbol: &Symbol) -> String {
  match symbol {
    &Symbol::Plus => String::from("+"),
    &Symbol::Minus => String::from("-"),
    &Symbol::Multiply => String::from("*"),
    &Symbol::Divide => String::from("/"),
    &Symbol::LParen => String::from("("),
    &Symbol::RParen => String::from(")"),
    &Symbol::LSquareBracket => String::from("["),
    &Symbol::RSquareBracket => String::from("]"),
    &Symbol::LCurlyBracket => String::from("{"),
    &Symbol::RCurlyBracket => String::from("}"),
    &Symbol::Space => String::from(" "),
    &Symbol::CR => String::from("\r"),
    &Symbol::LB => String::from("\n"),
    &Symbol::Arrow => String::from("->"),
    &Symbol::Comma => String::from(","),
    &Symbol::Underscore => String::from("_"),
    &Symbol::Colon => String::from(":"),
  }
}
