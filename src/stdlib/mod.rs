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

pub fn register(memory: &mut HashMap<&str, Constant>) {
  memory.insert("+", create_base_fn(add::add));
  memory.insert("-", create_base_fn(subtract::subtract));
  memory.insert("*", create_base_fn(multiply::multiply));
  memory.insert("/", create_base_fn(divide::divide));
  memory.insert("%", create_base_fn(modulo::modulo));
  memory.insert("&&", create_base_fn(and::and));
  memory.insert("||", create_base_fn(or::or));
  memory.insert("!", create_base_fn(not::not));
  memory.insert("len", create_base_fn(len::len));
  memory.insert("==", create_base_fn(eq::eq));
  memory.insert(">", create_base_fn(gt::gt));
  memory.insert(">=", create_base_fn(ge::ge));
  memory.insert("<", create_base_fn(lt::lt));
  memory.insert("<=", create_base_fn(le::le));
}

fn create_base_fn(fun: fn(Vec<Constant>) -> Constant) -> Constant {
  Constant::Function(Function {
    args: vec![],
    implementation: fun,
  })
}