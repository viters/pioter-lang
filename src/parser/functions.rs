use parser::debug_pair;
use parser::Function;
use parser::parse_p_eip;
use parser::primitives::parse_constant;
use pest::iterators::Pair;
use std::collections::HashMap;
use super::Constant;
use super::primitives;
use super::Rule;

pub fn parse_funcall(pair: Pair<Rule>, memory: &HashMap<&str, Constant>, local: Option<&Vec<&str>>) -> Constant {
  let inner = pair.clone().into_inner().nth(0).unwrap();

  match inner.as_rule() {
    Rule::p_sfuncall => {
      let fun = primitives::parse_constant(inner.clone().into_inner().nth(0).unwrap(), memory);
      let args = parse_funcall_args(inner.clone().into_inner().nth(1).unwrap(), memory, local);

      match local {
        Some(t) => Constant::Function(Function {
          args,
          argc: t.len(),
          base_fn: Some(Box::new(fun)),
          implementation: None,
        }),
        None => run_fn(fun, args)
      }
    }
    Rule::p_iifuncall => panic!(),
    _ => unreachable!()
  }
}

pub fn parse_fundef(pair: Pair<Rule>, memory: &HashMap<&str, Constant>) -> Constant {
  let mut args = vec![];
  let mut fun = None;

  for inner in pair.into_inner() {
    match inner.as_rule() {
      Rule::p_funparam => {
        for p_funparam_inner in inner.into_inner() {
          match p_funparam_inner.as_rule() {
            Rule::p_funparam2 => {
              for param in p_funparam_inner.into_inner() {
                args.push(param.into_span().as_str());
              }
            },
            _ => unreachable!()
          }
        }
      },
      Rule::p_eip => {
        fun = Some(parse_p_eip_fn(inner, memory, &args));
      },
      _ => unreachable!()
    }
  }

  fun.unwrap()
}

fn parse_funcall_args(pair: Pair<Rule>, memory: &HashMap<&str, Constant>, local: Option<&Vec<&str>>) -> Vec<Constant> {
  let inner = pair.clone().into_inner();

  let mut args;
  match local {
    Some(t) => args = vec![parse_p_eip_fn(inner.clone().nth(0).unwrap(), memory, t)],
    None => args = vec![super::parse_p_eip(inner.clone().nth(0).unwrap(), memory)]
  }

  if inner.clone().count() > 1 {
    args.append(parse_funcall_args(inner.clone().nth(1).unwrap(), memory, local).as_mut());
  }

  return args;
}

fn parse_p_eip_fn(pair: Pair<Rule>, memory: &HashMap<&str, Constant>, local: &Vec<&str>) -> Constant {
  match pair.as_rule() {
    Rule::constant => {
      let name = pair.clone().into_span().as_str();
      let index = local.iter().position(|&r| r == name);

      match index {
        Some(i) => Constant::Index(i),
        None => parse_constant(pair, memory)
      }
    },
    Rule::p_funcall => parse_funcall(pair, memory, Some(local)),
    Rule::p_eip => parse_p_eip_fn(pair.into_inner().nth(0).unwrap(), memory, local),
    _ => parse_p_eip(pair, memory)
  }
}

fn run_fn(constant: Constant, args: Vec<Constant>) -> Constant {
  match constant {
    Constant::Function(fun) => {
      let argc = args.len();
      let diff = fun.argc - argc;


      let new_args = fun.args.into_iter().map(|a| match a {
        Constant::Index(i) => {
          let v = args.get(i);

          match v {
            Some(t) => t.clone(),
            None => Constant::Index(i - argc)
          }
        },
        _ => a
      }).collect();

      if diff == 0 {
        match fun.base_fn {
          Some(t) => {
            run_fn(unbox(t), new_args)
          },
          None => (fun.implementation.unwrap())(new_args)
        }
      } else if diff > 0 {
        Constant::Function(Function {
          args: new_args,
          argc: diff,
          base_fn: fun.base_fn,
          implementation: fun.implementation,
        })
      } else {
        eprintln!("Too many arguments passed.");
        panic!()
      }
    },
    _ => {
      eprintln!("Tried to call a constant value \"{:?}\" instead of function!", constant);
      panic!();
    }
  }
}

fn unbox<T>(value: Box<T>) -> T {
  *value
}