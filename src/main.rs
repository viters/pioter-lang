use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use tokenizer::*;
use tokenizer::keyword::*;
use tokenizer::symbol::*;

mod tokenizer;

fn main() {
  let mut fo = File::open("code.ptr").expect("File not found!");
  let mut contents = String::new();
  fo.read_to_string(&mut contents).expect("Something went wrong while reading");

  let tokens = contents.tokenize().unwrap();
  let code: String = tokens.iter().map(|token| match token {
    &Token::Keyword(ref keyword) => ["<span style=\"color: #8e44ad; font-weight: 700\">", &keyword_to_string(&keyword), "</span>"].join(""),
    &Token::Integer(i) => ["<span style=\"color: #e67e22\">", &i.to_string(), "</span>"].join(""),
    &Token::Operator(ref symbol) => match symbol {
      &Symbol::LB => String::from("<br>"),
      _ => ["<span style=\"color: #3498db; font-weight: 700\">", &symbol_to_string(&symbol), "</span>"].join("")
    },
    &Token::Variable(ref var) => ["<span style=\"color: #e74c3c; font-weight: 700\">", var, "</span>"].join(""),
    &Token::String(ref var) => ["<span style=\"color: #2ecc71; font-weight: 700\">'", var, "'</span>"].join("")
  }).collect();

  let mut f = File::create("code.html").expect("Unable to create file!");
  f.write_all(code.as_bytes()).expect("Unable to write to file!");
}
