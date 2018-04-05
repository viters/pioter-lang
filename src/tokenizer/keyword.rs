#[derive(Debug, PartialEq)]
pub enum Keyword {
  Def,
  True,
  False,
}

pub fn keyword_to_string(keyword: &Keyword) -> String {
  match keyword {
    &Keyword::Def => String::from("def"),
    &Keyword::True => String::from("true"),
    &Keyword::False => String::from("false"),
  }
}