use parser::functions::*;
use parser::lists::*;
use parser::primitives::*;
use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;
use super::stdlib;

mod primitives;
mod functions;
mod lists;

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
  pub args: Vec<Constant>,
  pub base_fn: Option<Box<Constant>>,
  pub implementation: Option<fn(Vec<Constant>) -> Constant>,
  pub argc: usize
}

#[derive(Debug, PartialEq, Clone)]
pub enum Constant {
  Float(f32),
  Integer(i32),
  String(String),
  Boolean(bool),
  Function(Function),
  List(Vec<Constant>),
  Index(usize)
}

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
struct PioterParser;

pub fn parse(contents: String) {
  let p_starts = PioterParser::parse(Rule::p_start, &contents).unwrap_or_else(|e| panic!("{}", e));

  let mut memory: HashMap<&str, Constant> = HashMap::new();
  stdlib::register(&mut memory);

  for p_start in p_starts {
    for p_def in p_start.into_inner() {
      match p_def.as_rule() {
        Rule::p_dump => {
          let pair = p_def.into_inner().nth(0).unwrap();
          let name = pair.clone().into_span().as_str();
          let constant = parse_p_eip(pair, &memory);
          println!("{} = {:?}", name, constant)
        }
        Rule::p_def => {
          let mut name = "";

          for p_def_inner in p_def.into_inner() {
            match p_def_inner.as_rule() {
              Rule::def => (),
              Rule::constant => {
                name = p_def_inner.into_span().as_str();

                if memory.get(name).is_some() {
                  println!("Constant \"{}\" is already defined!", name);
                  panic!();
                }
              }
              Rule::p_eip => {
                for p_eip_inner in p_def_inner.into_inner() {
                  let value = parse_p_eip(p_eip_inner, &memory);
                  memory.insert(name, value);
                }
              }
              _ => unreachable!()
            }
          }
        }
        _ => unreachable!()
      }
    }
  }
}

pub fn parse_p_eip(pair: Pair<Rule>, memory: &HashMap<&str, Constant>) -> Constant {
  match pair.as_rule() {
    Rule::number => parse_number(pair.into_inner().nth(0).unwrap()),
    Rule::string => parse_string(pair),
    Rule::boolean => parse_boolean(pair.into_inner().nth(0).unwrap()),
    Rule::constant => parse_constant(pair, memory),
    Rule::p_funcall => parse_funcall(pair, memory, None),
    Rule::p_fundef => parse_fundef(pair, memory),
    Rule::p_eip => parse_p_eip(pair.into_inner().nth(0).unwrap(), memory),
    Rule::p_list => parse_p_list(pair.into_inner().nth(0).unwrap(), memory),
    _ => unreachable!()
  }
}

#[allow(dead_code)]
pub fn debug_pair(pair: Pair<Rule>) {
  let span = pair.clone().into_span();
  println!("Rule:    {:?}", pair.as_rule());
  println!("Span:    {:?}", span);
  println!("Text:    {}", span.as_str());
}
