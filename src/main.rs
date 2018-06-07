extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::collections::HashMap;
use pest::iterators::Pair;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct PioterParser;

#[derive(Debug, PartialEq, Clone)]
enum Constant {
  Float(f32),
  Integer(i32),
  String(String),
  Boolean(bool),
}

fn main() {
  let contents = read_file("code2.ptr");
  tokenize_string(contents);

//  write_to_file("code.html", code);
}

fn read_file(name: &str) -> String {
  let mut fo = File::open(name).expect("File not found!");
  let mut contents = String::new();
  fo.read_to_string(&mut contents).expect("Something went wrong while reading");

  contents
}

fn tokenize_string(contents: String) {
  let p_starts = PioterParser::parse(Rule::p_start, &contents).unwrap_or_else(|e| panic!("{}", e));

  let mut memory: HashMap<&str, Constant> = HashMap::new();

  for p_start in p_starts {
    for p_def in p_start.into_inner() {
      match p_def.as_rule() {
        Rule::p_def => {
          let mut name = "";

          for p_def_inner in p_def.into_inner() {
            match p_def_inner.as_rule() {
              Rule::def => (),
              Rule::constant => {
                name = p_def_inner.into_span().as_str();
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

  for (key, value) in memory {
    println!("{} / {:?}", key, value);
  }
}

fn parse_p_eip(pair: Pair<Rule>, memory: &HashMap<&str, Constant>) -> Constant {
  match pair.as_rule() {
    Rule::number => parse_number(pair.into_inner().nth(0).unwrap()),
    Rule::string => parse_string(pair),
    Rule::boolean => parse_boolean(pair.into_inner().nth(0).unwrap()),
    Rule::constant => parse_constant(pair, memory),
    Rule::p_eip => parse_p_eip(pair.into_inner().nth(0).unwrap(), memory),
    _ => unreachable!()
  }
}

fn parse_number(pair: Pair<Rule>) -> Constant {
  match pair.as_rule() {
    Rule::float => Constant::Float(pair.into_span().as_str().parse::<f32>().unwrap()),
    Rule::integer => Constant::Integer(pair.into_span().as_str().parse::<i32>().unwrap()),
    _ => unreachable!()
  }
}

fn parse_boolean(pair: Pair<Rule>) -> Constant {
  match pair.as_rule() {
    Rule::k_true => Constant::Boolean(true),
    Rule::k_false => Constant::Boolean(false),
    _ => unreachable!()
  }
}

fn parse_string(pair: Pair<Rule>) -> Constant {
  Constant::String(String::from(pair.into_inner().nth(0).unwrap().into_span().as_str()))
}

fn parse_constant(pair: Pair<Rule>, memory: &HashMap<&str, Constant>) -> Constant {
  memory.get(pair.into_span().as_str()).unwrap().clone()
}

#[allow(dead_code)]
fn write_to_file(name: &str, contents: String) {
  let mut f = File::create(name).expect("Unable to create file!");
  f.write_all(contents.as_bytes()).expect("Unable to write to file!");
}