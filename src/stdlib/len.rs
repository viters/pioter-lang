use super::Constant;

pub fn len(args: Vec<Constant>) -> Constant {
  let arg = args.get(0).unwrap();

  match arg {
    &Constant::List(ref l) => Constant::Integer(l.len() as i32),
    _ => {
      eprintln!("Invalid arguments passed to \"len\"!");
      panic!()
    }
  }
}