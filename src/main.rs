use std::iter::Peekable;
use std::str::Chars;
use std::fs::File;
use std::io::Write;

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
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Integer(i32),
    Operator(Symbol),
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
                        tokens.push(Token::Integer(num.parse::<i32>().unwrap()));
                    }
                    '+' => {
                        it.next().unwrap();
                        tokens.push(Token::Operator(Symbol::Plus));
                    }
                    '-' => {
                        it.next().unwrap();
                        tokens.push(Token::Operator(Symbol::Minus));
                    }
                    '*' => {
                        it.next().unwrap();
                        tokens.push(Token::Operator(Symbol::Multiply));
                    }
                    '/' => {
                        it.next().unwrap();
                        tokens.push(Token::Operator(Symbol::Divide));
                    },
                    '(' => {
                        it.next().unwrap();
                        tokens.push(Token::Operator(Symbol::LParen));
                    },
                    ')' => {
                        it.next().unwrap();
                        tokens.push(Token::Operator(Symbol::RParen));
                    },
                    '[' => {
                        it.next().unwrap();
                        tokens.push(Token::Operator(Symbol::LSquareBracket));
                    }
                    ']' => {
                        it.next().unwrap();
                        tokens.push(Token::Operator(Symbol::RSquareBracket));
                    },
                    ' ' => {
                        it.next().unwrap();
                        tokens.push(Token::Operator(Symbol::Space));
                    },
                    '\r' => {
                        it.next().unwrap();
                        tokens.push(Token::Operator(Symbol::CR));
                    },
                    '\n' => {
                        it.next().unwrap();
                        tokens.push(Token::Operator(Symbol::LB));
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
    let tokens = String::from("/ (+ 5 2) 7 \n * 2 7").tokenize().unwrap();
    let code: String = tokens.iter().map(|token| match token {
        &Token::Integer(i) => ["<span style=\"color: #e67e22\">", &i.to_string(), "</span>"].join(""),
        &Token::Operator(ref symbol) => match symbol {
            &Symbol::LB => String::from("<br>"),
            _ => ["<span style=\"color: #3498db; font-weight: 700\">", &symbol_to_string(&symbol), "</span>"].join("")
        }
    }).collect();
    let mut f = File::create("code.html").expect("Unable to create file!");
    f.write_all(code.as_bytes()).expect("Unable to write to file!");
}
