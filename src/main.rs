extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::iterators::Pair;
use pest::Parser;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct PioterParser;

struct Function {
  name: String,
}

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
    Rule::p_funcall => parse_funcall(pair, memory),
    Rule::p_fundef => parse_fundef(pair),
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
  let const_name = pair.into_span().as_str();
  let value = memory.get(const_name);

  if value.is_none() {
    println!("Constant \"{}\" was not defined!", const_name);
    panic!()
  }

  value.unwrap().clone()
}

fn parse_funcall(pair: Pair<Rule>, memory: &HashMap<&str, Constant>) -> Constant {
  let inner = pair.clone().into_inner().nth(0).unwrap();

  match inner.as_rule() {
    Rule::p_sfuncall => {
      let fun = parse_constant(inner.clone().into_inner().nth(0).unwrap(), memory);
      let args = parse_funcall_args(inner.clone().into_inner().nth(1).unwrap(), memory);

      println!("Fn called {:?}", fun);
      println!("Fn args passed {:?}", args);
    }
    Rule::p_iifuncall => {}
    _ => unreachable!()
  }

  Constant::String(String::from(pair.into_span().as_str()))
}

fn parse_funcall_args(pair: Pair<Rule>, memory: &HashMap<&str, Constant>) -> Vec<Constant> {
  let inner = pair.clone().into_inner();

  let mut args = vec![parse_p_eip(inner.clone().nth(0).unwrap(), memory)];

  if inner.clone().count() > 1 {
    args.append(parse_funcall_args(inner.clone().nth(1).unwrap(), memory).as_mut());
  }

  return args;
}

fn parse_fundef(pair: Pair<Rule>) -> Constant {
  Constant::String(String::from("Function"))
}

#[allow(dead_code)]
fn debug_pair(pair: Pair<Rule>) {
  let span = pair.clone().into_span();
  println!("Rule:    {:?}", pair.as_rule());
  println!("Span:    {:?}", span);
  println!("Text:    {}", span.as_str());
}

#[allow(dead_code)]
fn write_to_file(name: &str, contents: String) {
  let mut f = File::create(name).expect("Unable to create file!");
  f.write_all(contents.as_bytes()).expect("Unable to write to file!");
}