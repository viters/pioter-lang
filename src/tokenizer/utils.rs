use std::iter::Peekable;
use std::str::Chars;

pub fn consume_while<F>(it: &mut Peekable<Chars>, pred: F) -> Vec<char>
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