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
  memory.insert("+", create_stdlib_fn(add::add));
  memory.insert("-", create_stdlib_fn(subtract::subtract));
  memory.insert("*", create_stdlib_fn(multiply::multiply));
  memory.insert("/", create_stdlib_fn(divide::divide));
  memory.insert("%", create_stdlib_fn(modulo::modulo));
  memory.insert("&&", create_stdlib_fn(and::and));
  memory.insert("||", create_stdlib_fn(or::or));
  memory.insert("!", create_stdlib_fn(not::not));
  memory.insert("len", create_stdlib_fn(len::len));
  memory.insert("==", create_stdlib_fn(eq::eq));
  memory.insert(">", create_stdlib_fn(gt::gt));
  memory.insert(">=", create_stdlib_fn(ge::ge));
  memory.insert("<", create_stdlib_fn(lt::lt));
  memory.insert("<=", create_stdlib_fn(le::le));
  memory.insert("nth", create_stdlib_fn(nth::nth));
}

fn create_stdlib_fn(fun: fn(Vec<Constant>) -> Constant) -> Constant {
  Constant::Function(Function {
    args: vec![],
    base_fn: None,
    implementation: Some(fun),
  })
}