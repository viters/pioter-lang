use std::collections::HashMap;
use super::parser::Constant;
use super::parser::Function;

pub mod add;
pub mod subtract;
pub mod multiply;
pub mod divide;
pub mod modulo;
pub mod and;
pub mod or;

pub fn register(memory: &mut HashMap<&str, Constant>) {
  memory.insert("+", create_base_fn(add::add));
  memory.insert("-", create_base_fn(subtract::subtract));
  memory.insert("*", create_base_fn(multiply::multiply));
  memory.insert("/", create_base_fn(divide::divide));
  memory.insert("mod", create_base_fn(modulo::modulo));
  memory.insert("and", create_base_fn(and::and));
  memory.insert("or", create_base_fn(or::or));
}

fn create_base_fn(fun: fn(Vec<Constant>) -> Constant) -> Constant {
  Constant::Function(Function {
    args: vec![],
    implementation: fun,
  })
}