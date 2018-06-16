use super::Constant;

pub fn and(args: Vec<Constant>) -> Constant {
  let pair: (&Constant, &Constant) = (args.get(0).unwrap(), args.get(1).unwrap());

  match pair {
    (&Constant::Boolean(v1), &Constant::Boolean(v2)) => Constant::Boolean(v1 && v2),
    _ => {
      eprintln!("Invalid arguments passed to \"and\"!");
      panic!()
    }
  }
}