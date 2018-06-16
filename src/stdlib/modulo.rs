use super::Constant;

pub fn modulo(args: Vec<Constant>) -> Constant {
  let pair: (&Constant, &Constant) = (args.get(0).unwrap(), args.get(1).unwrap());

  match pair {
    (&Constant::Integer(v1), &Constant::Integer(v2)) => Constant::Integer(v1 % v2),
    _ => {
      eprintln!("Invalid arguments passed to \"mod\"!");
      panic!()
    }
  }
}