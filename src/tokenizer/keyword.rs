#[derive(Debug, PartialEq)]
pub enum Keyword {
  Def,
}

pub fn keyword_to_string(keyword: &Keyword) -> String {
  match keyword {
    &Keyword::Def => String::from("def"),
  }
}