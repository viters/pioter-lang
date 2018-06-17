use super::Constant;

pub fn not(args: Vec<Constant>) -> Constant {
  let arg = args.get(0).unwrap();

  match arg {
    &Constant::Boolean(b) => Constant::Boolean(!b),
    _ => {
      eprintln!("Invalid arguments passed to \"not\"!");
      panic!()
    }
  }
}