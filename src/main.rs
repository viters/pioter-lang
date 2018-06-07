extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct PioterParser;

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

  let mut variables: HashMap<&str, &str> = HashMap::new();

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
              },
              Rule::p_eip => {
                for p_eip_inner in p_def_inner.into_inner() {
                  match p_eip_inner.as_rule() {
                    Rule::number => variables.insert(name, p_eip_inner.into_span().as_str()),
                    Rule::string => variables.insert(name, p_eip_inner.into_span().as_str()),
                    Rule::boolean => variables.insert(name, p_eip_inner.into_span().as_str()),
                    Rule::constant => variables.insert(name, p_eip_inner.into_span().as_str()),
                    _ => variables.insert(name, p_eip_inner.into_span().as_str())
                  }
                }
              },
              _ => unreachable!()
            }
          }
        },
        _ => unreachable!()
      }
    }
  }

  for (key, value) in variables {
    println!("{} / {}", key, value);
  }
}

#[allow(dead_code)]
fn write_to_file(name: &str, contents: String) {
  let mut f = File::create(name).expect("Unable to create file!");
  f.write_all(contents.as_bytes()).expect("Unable to write to file!");
}