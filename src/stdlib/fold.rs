use super::Constant;
use super::run_fn;

pub fn fold(args: Vec<Constant>) -> Constant {
  let pair: (&Constant, &Constant) = (args.get(1).unwrap(), args.get(2).unwrap());

  match pair {
    (&Constant::Function(ref v1), &Constant::List(ref v2)) => {
      let mut folded = args.get(0).unwrap().clone();

      for e in v2 {
        folded = run_fn(Constant::Function(v1.clone()), vec![folded, e.clone()]);
      }

      folded
    }
    _ => {
      eprintln!("Invalid arguments passed to \"fold\"!");
      panic!()
    }
  }
}