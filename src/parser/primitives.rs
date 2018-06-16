use pest::iterators::Pair;
use std::collections::HashMap;
use super::Constant;
use super::Rule;

pub fn parse_number(pair: Pair<Rule>) -> Constant {
  match pair.as_rule() {
    Rule::float => Constant::Float(pair.into_span().as_str().parse::<f32>().unwrap()),
    Rule::integer => Constant::Integer(pair.into_span().as_str().parse::<i32>().unwrap()),
    _ => unreachable!()
  }
}

pub fn parse_boolean(pair: Pair<Rule>) -> Constant {
  match pair.as_rule() {
    Rule::k_true => Constant::Boolean(true),
    Rule::k_false => Constant::Boolean(false),
    _ => unreachable!()
  }
}

pub fn parse_string(pair: Pair<Rule>) -> Constant {
  Constant::String(String::from(pair.into_inner().nth(0).unwrap().into_span().as_str()))
}

pub fn parse_constant(pair: Pair<Rule>, memory: &HashMap<&str, Constant>) -> Constant {
  let const_name = pair.into_span().as_str();
  let value = memory.get(const_name);

  if value.is_none() {
    eprintln!("Constant \"{}\" was not defined!", const_name);
    panic!()
  }

  value.unwrap().clone()
}