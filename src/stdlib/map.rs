use super::Constant;
use super::run_fn;

pub fn map(args: Vec<Constant>) -> Constant {
  let pair: (&Constant, &Constant) = (args.get(0).unwrap(), args.get(1).unwrap());

  match pair {
    (&Constant::Function(ref v1), &Constant::List(ref v2)) =>
      Constant::List(v2.into_iter().map(|a| run_fn(Constant::Function(v1.clone()), vec![a.clone()])).collect()),
    _ => {
      eprintln!("Invalid arguments passed to \"map\"!");
      panic!()
    }
  }
}