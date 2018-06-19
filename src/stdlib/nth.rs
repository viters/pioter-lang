use super::Constant;

pub fn nth(args: Vec<Constant>) -> Constant {
  let pair: (&Constant, &Constant) = (args.get(0).unwrap(), args.get(1).unwrap());

  match pair {
    (&Constant::Integer(v1), &Constant::List(ref v2)) => {
      let v = v2.get(v1 as usize);

      match v {
        Some(x) => x.clone(),
        None => {
          eprintln!("Index out of bounds!");
          panic!()
        }
      }
    }
    _ => {
      eprintln!("Invalid arguments passed to \"nth\"!");
      panic!()
    }
  }
}