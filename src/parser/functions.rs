use parser::debug_pair;
use pest::iterators::Pair;
use std::collections::HashMap;
use super::Constant;
use super::primitives;
use super::Rule;

pub fn parse_funcall(pair: Pair<Rule>, memory: &HashMap<&str, Constant>) -> Constant {
  let inner = pair.clone().into_inner().nth(0).unwrap();

  match inner.as_rule() {
    Rule::p_sfuncall => {
      let fun = primitives::parse_constant(inner.clone().into_inner().nth(0).unwrap(), memory);
      let args = parse_funcall_args(inner.clone().into_inner().nth(1).unwrap(), memory);

      return run_fn(fun, args);
    }
    Rule::p_iifuncall => panic!(),
    _ => unreachable!()
  }
}

pub fn parse_fundef(pair: Pair<Rule>) -> Constant {
  for inner in pair.into_inner() {
    debug_pair(inner);
  }

  Constant::Integer(42)
}

fn parse_funcall_args(pair: Pair<Rule>, memory: &HashMap<&str, Constant>) -> Vec<Constant> {
  let inner = pair.clone().into_inner();

  let mut args = vec![super::parse_p_eip(inner.clone().nth(0).unwrap(), memory)];

  if inner.clone().count() > 1 {
    args.append(parse_funcall_args(inner.clone().nth(1).unwrap(), memory).as_mut());
  }

  return args;
}

fn run_fn(constant: Constant, args: Vec<Constant>) -> Constant {
  match constant {
    Constant::Function(fun) => return (fun.implementation)(args),
    _ => {
      eprintln!("Tried to call a constant value \"{:?}\" instead of function!", constant);
      panic!();
    }
  }

//  let mut next_provided_arg = args.clone().iter();
//  let full_args: Vec<Option<Constant>> = fun.args.clone().into_iter().map(|f| match f {
//    Some(T) => Some(T),
//    None => next_provided_arg.next().cloned()
//  }).collect();
//
//  if full_args.into_iter().filter(|a| a.is_none()).count() == 0 {
//    let mut next_args_num = fun.children_args.clone().iter();
//
//    return (fun.implementation)(
//      fun.children.into_iter()
//        .map(|child|
//          match child {
//            Some(T) => match T {
//              Constant::Function(F) =>
//                run_fn(F, next_args_num.next().unwrap().into_iter().map(|i| full_args[i.clone()].unwrap()).collect()),
//              _ => panic!()
//            },
//            None => full_args.clone().into_iter().nth(
//              next_args_num.next().unwrap().get(0).unwrap().clone()
//            ).unwrap()
//          }).collect()
//    );
//  } else {
//    return Constant::Function(Function {
//      args: full_args,
//      children: fun.children,
//      children_args: fun.children_args,
//      implementation: fun.implementation
//    });
//  }
}
