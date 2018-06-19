use super::Constant;
use super::run_fn;

pub fn filter(args: Vec<Constant>) -> Constant {
  let pair: (&Constant, &Constant) = (args.get(0).unwrap(), args.get(1).unwrap());

  match pair {
    (&Constant::Function(ref v1), &Constant::List(ref v2)) => {
      let mut filtered_vec = vec![];

      for e in v2 {
        match run_fn(Constant::Function(v1.clone()), vec![e.clone()]) {
          Constant::Boolean(b) => {
            if b {
              filtered_vec.push(e.clone())
            }
          }
          _ => {
            eprintln!("Function passed to filter should return boolean value!");
            panic!()
          }
        }
      }

      Constant::List(filtered_vec)
    }
    _ => {
      eprintln!("Invalid arguments passed to \"map\"!");
      panic!()
    }
  }
}