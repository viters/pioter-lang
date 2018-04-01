use std::iter::Peekable;
use std::str::Chars;
use std::fs::File;
use std::io::Write;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
enum Symbol {
    Plus,
    Minus,
    Multiply,
    Divide,
    LParen,
    RParen,
    LSquareBracket,
    RSquareBracket,
    Space,
    CR,
    LB,
    Arrow,
}

fn symbol_to_string(symbol: &Symbol) -> String {
    match symbol {
        &Symbol::Plus => String::from("+"),
        &Symbol::Minus => String::from("-"),
        &Symbol::Multiply => String::from("*"),
        &Symbol::Divide => String::from("/"),
        &Symbol::LParen => String::from("("),
        &Symbol::RParen => String::from(")"),
        &Symbol::LSquareBracket => String::from("["),
        &Symbol::RSquareBracket => String::from("]"),
        &Symbol::Space => String::from(" "),
        &Symbol::CR => String::from("\r"),
        &Symbol::LB => String::from("\n"),
        &Symbol::Arrow => String::from("->"),
    }
}

#[derive(Debug, PartialEq)]
enum Keyword {
    Def,
}

fn keyword_to_string(keyword: &Keyword) -> String {
    match keyword {
        &Keyword::Def => String::from("def"),
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Integer(i32),
    Operator(Symbol),
    Keyword(Keyword),
    Variable(String),
    String(String)
}

trait Tokenizer {
    fn tokenize(&self) -> Result<Vec<Token>, &'static str>;
}

impl Tokenizer for String {
    fn tokenize(&self) -> Result<Vec<Token>, &'static str> {
        let mut it = self.chars().peekable();
        let mut tokens: Vec<Token> = vec![];
        loop {
            match it.peek() {
                Some(&ch) => match ch {
                    '0' ... '9' => {
                        let num: String = consume_while(&mut it, |a| a.is_numeric())
                            .into_iter()
                            .collect();
                        if it.peek().unwrap().is_alphabetic() {
                            panic!("Syntax error");
                        }

                        tokens.push(Token::Integer(num.parse::<i32>().unwrap()));
                    }
                    '+' => {
                        it.next();
                        tokens.push(Token::Operator(Symbol::Plus));
                    }
                    '-' => {
                        match it.peek().unwrap() {
                            '>' => {
                                it.next();
                                it.next();
                                tokens.push(Token::Operator(Symbol::Arrow));
                            },
                            _ => {
                                it.next();
                                tokens.push(Token::Operator(Symbol::Minus));
                            }
                        }
                    }
                    '*' => {
                        it.next();
                        tokens.push(Token::Operator(Symbol::Multiply));
                    }
                    '/' => {
                        it.next();
                        tokens.push(Token::Operator(Symbol::Divide));
                    },
                    '(' => {
                        it.next();
                        tokens.push(Token::Operator(Symbol::LParen));
                    },
                    ')' => {
                        it.next();
                        tokens.push(Token::Operator(Symbol::RParen));
                    },
                    '[' => {
                        it.next();
                        tokens.push(Token::Operator(Symbol::LSquareBracket));
                    }
                    ']' => {
                        it.next();
                        tokens.push(Token::Operator(Symbol::RSquareBracket));
                    },
                    ' ' => {
                        it.next();
                        tokens.push(Token::Operator(Symbol::Space));
                    },
                    '\r' => {
                        it.next();
                        tokens.push(Token::Operator(Symbol::CR));
                    },
                    '\n' => {
                        it.next();
                        tokens.push(Token::Operator(Symbol::LB));
                    },
                    '\'' => {
                        it.next();
                        let chars: String = consume_while(&mut it, |a| a != '\'')
                            .into_iter()
                            .collect();
                        it.next();

                        tokens.push(Token::String(chars));
                    },
                    'A' ... 'Z' | 'a' ... 'z' | '_' => {
                        let chars: String = consume_while(&mut it, |a| a.is_alphanumeric() || a == '_')
                            .into_iter()
                            .collect();

                        match chars.as_ref() {
                            "def" => tokens.push(Token::Keyword(Keyword::Def)),
                            _ => tokens.push(Token::Variable(chars))
                        }
                    },
                    _ => panic!("Invalid char!")
                },
                None => break
            }
        }

        Ok(tokens)
    }
}

fn consume_while<F>(it: &mut Peekable<Chars>, pred: F) -> Vec<char>
    where F: Fn(char) -> bool {
    let mut chars: Vec<char> = vec![];

    while let Some(&ch) = it.peek() {
        if pred(ch) {
            it.next().unwrap();
            chars.push(ch);
        } else {
            break;
        }
    }

    chars
}

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
