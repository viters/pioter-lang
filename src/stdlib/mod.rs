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
mod len;

pub fn register(memory: &mut HashMap<&str, Constant>) {
  memory.insert("+", create_base_fn(add::add));
  memory.insert("-", create_base_fn(subtract::subtract));
  memory.insert("*", create_base_fn(multiply::multiply));
  memory.insert("/", create_base_fn(divide::divide));
  memory.insert("mod", create_base_fn(modulo::modulo));
  memory.insert("and", create_base_fn(and::and));
  memory.insert("or", create_base_fn(or::or));
  memory.insert("len", create_base_fn(len::len));
}

fn create_base_fn(fun: fn(Vec<Constant>) -> Constant) -> Constant {
  Constant::Function(Function {
    args: vec![],
    implementation: fun,
  })
}