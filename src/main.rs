extern crate pest;
#[macro_use]
extern crate pest_derive;

use parser::parse;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use tokenizer::tokenize;

mod tokenizer;
mod parser;
mod stdlib;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.get(0).is_none() {
    eprintln!("Please provide file name as parameter");
    panic!();
  }

  let contents = read_file(args.get(1).unwrap());
  let tokens = tokenize(&contents);
  if tokens.is_err() {
    let errors = tokens.err().unwrap();
    errors.iter().for_each(|err| eprintln!("ERROR: {}", err));
    panic!()
  }

  parse(contents);
}

fn read_file(name: &str) -> String {
  let mut fo = File::open(name).expect("File not found!");
  let mut contents = String::new();
  fo.read_to_string(&mut contents).expect("Something went wrong while reading");

  contents
}
