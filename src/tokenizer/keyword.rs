#[derive(Debug, PartialEq)]
pub enum Keyword {
  Def,
  True,
  False,
  Match,
}

pub fn keyword_to_string(keyword: &Keyword) -> String {
  match keyword {
    &Keyword::Def => String::from("def"),
    &Keyword::True => String::from("true"),
    &Keyword::False => String::from("false"),
    &Keyword::Match => String::from("match"),
  }
}