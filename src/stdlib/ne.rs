use super::Constant;

pub fn ne(args: Vec<Constant>) -> Constant {
  super::not::not(vec![super::eq::eq(args)])
}