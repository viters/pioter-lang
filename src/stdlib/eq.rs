use super::Constant;

pub fn eq(args: Vec<Constant>) -> Constant {
  let pair: (&Constant, &Constant) = (args.get(0).unwrap(), args.get(1).unwrap());

  match pair {
    (&Constant::Float(v1), &Constant::Float(v2)) => Constant::Boolean(v1 == v2),
    (&Constant::Float(v1), &Constant::Integer(v2)) => Constant::Boolean(v1 == (v2 as f32)),
    (&Constant::Integer(v1), &Constant::Integer(v2)) => Constant::Boolean(v1 == v2),
    (&Constant::Integer(v1), &Constant::Float(v2)) => Constant::Boolean((v1 as f32) == v2),
    (&Constant::Boolean(v1), &Constant::Boolean(v2)) => Constant::Boolean(v1 == v2),
    (&Constant::String(ref v1), &Constant::String(ref v2)) => Constant::Boolean(v1.to_owned() == v2.to_owned()),
    _ => {
      eprintln!("Invalid arguments passed to \"==\"!");
      panic!()
    }
  }
}