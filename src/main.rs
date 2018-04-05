use std::fs::File;
use std::io::prelude::*;
use std::io::Write;
use tokenizer::*;
use tokenizer::keyword::*;
use tokenizer::symbol::*;

mod tokenizer;

fn main() {
  let contents = read_file("code.ptr");
  let tokens = tokenize_string(contents);
  let code = tokens_to_html(tokens.unwrap());

  write_to_file("code.html", code);
}

fn read_file(name: &str) -> String {
  let mut fo = File::open(name).expect("File not found!");
  let mut contents = String::new();
  fo.read_to_string(&mut contents).expect("Something went wrong while reading");

  contents
}

fn tokenize_string(contents: String) -> Result<Vec<Token>, Vec<String>> {
  let tokens = contents.tokenize();
  if tokens.is_err() {
    let errors = tokens.err().unwrap();
    errors.iter().for_each(|err| println!("ERROR: {}", err));
    panic!("There were errors during parsing");
  }

  tokens
}

fn tokens_to_html(tokens: Vec<Token>) -> String {
  tokens.iter().map(|token| match token {
    &Token::Keyword(ref keyword) => wrap_into_span("#8e44ad", keyword_to_string(&keyword).as_ref()),
    &Token::Integer(i) => wrap_into_span("#e67e22", i.to_string().as_ref()),
    &Token::Float(f) => wrap_into_span("#e67e22", f.to_string().as_ref()),
    &Token::Operator(ref symbol) => match symbol {
      &Symbol::LB => String::from("<br>"),
      _ => wrap_into_span("#3498db", symbol_to_string(&symbol).as_ref()),
    },
    &Token::Variable(ref x) => wrap_into_span("#e74c3c", x),
    &Token::String(ref x) => wrap_into_span("#2ecc71", ["'", x, "'"].join("").as_ref()),
  }).collect()
}

fn wrap_into_span(color: &str, content: &str) -> String {
  ["<span style=\"color: ", color, "\">", content, "</span>"].join("")
}

fn write_to_file(name: &str, contents: String) {
  let mut f = File::create(name).expect("Unable to create file!");
  f.write_all(contents.as_bytes()).expect("Unable to write to file!");
}