use pest::iterators::Pair;
use std::collections::HashMap;
use super::*;
use super::Constant;
use super::primitives;
use super::Rule;

pub fn parse_p_list(pair: Pair<Rule>, memory: &HashMap<&str, Constant>) -> Constant {
  let mut values: Vec<Constant> = Vec::new();
  for value in pair.into_inner() {
    let val = parse_p_eip(value.into_inner().nth(0).unwrap(), memory);
    values.push(val);
  }
  Constant::List(values)
}