use std::collections::HashMap;
use super::parser::Constant;
use super::parser::Function;

mod add;
mod subtract;
mod multiply;
mod divide;
mod modulo;
mod and;
mod or;
mod not;
mod len;
mod lt;
mod le;
mod eq;
mod ge;
mod gt;
mod nth;

pub fn register(memory: &mut HashMap<&str, Constant>) {
  memory.insert("+", create_stdlib_fn(add::add, 2));
  memory.insert("-", create_stdlib_fn(subtract::subtract, 2));
  memory.insert("*", create_stdlib_fn(multiply::multiply, 2));
  memory.insert("/", create_stdlib_fn(divide::divide, 2));
  memory.insert("%", create_stdlib_fn(modulo::modulo, 2));
  memory.insert("&&", create_stdlib_fn(and::and, 2));
  memory.insert("||", create_stdlib_fn(or::or, 2));
  memory.insert("!", create_stdlib_fn(not::not, 1));
  memory.insert("len", create_stdlib_fn(len::len, 1));
  memory.insert("==", create_stdlib_fn(eq::eq, 2));
  memory.insert(">", create_stdlib_fn(gt::gt, 2));
  memory.insert(">=", create_stdlib_fn(ge::ge, 2));
  memory.insert("<", create_stdlib_fn(lt::lt, 2));
  memory.insert("<=", create_stdlib_fn(le::le, 2));
  memory.insert("nth", create_stdlib_fn(nth::nth, 2));
}

fn create_stdlib_fn(fun: fn(Vec<Constant>) -> Constant, argc: usize) -> Constant {
  let args = (0..argc).map(|i| Constant::Index(i)).collect();

  Constant::Function(Function {
    args,
    argc,
    base_fn: None,
    implementation: Some(fun),
  })
}