use super::Constant;

pub fn subtract(args: Vec<Constant>) -> Constant {
  let pair: (&Constant, &Constant) = (args.get(0).unwrap(), args.get(1).unwrap());

  match pair {
    (&Constant::Float(v1), &Constant::Float(v2)) => Constant::Float(v1 - v2),
    (&Constant::Float(v1), &Constant::Integer(v2)) => Constant::Float(v1 - (v2 as f32)),
    (&Constant::Integer(v1), &Constant::Integer(v2)) => Constant::Integer(v1 - v2),
    (&Constant::Integer(v1), &Constant::Float(v2)) => Constant::Float((v1 as f32) - v2),
    _ => {
      eprintln!("Invalid arguments passed to \"-\"!");
      panic!()
    }
  }
}