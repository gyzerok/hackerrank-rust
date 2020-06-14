use num::bigint::BigInt;
use std::io;
use std::io::prelude::*;

pub fn main() {
  let reader = io::stdin();
  let n: u32 = readline(&reader).parse().unwrap();
  println!("{}", run(n));
}

fn run(n: u32) -> BigInt {
  let mut factorial: BigInt = BigInt::from(1);
  for i in 2..n {
    factorial *= i;
  }
  factorial * n
}

fn readline(reader: &std::io::Stdin) -> String {
  return reader
    .lock()
    .lines()
    .next()
    .unwrap()
    .ok()
    .unwrap()
    .trim()
    .to_string();
}

#[cfg(test)]
mod tests {
  use num::bigint::BigInt;

  #[test]
  fn it_works() {
    assert_eq!(BigInt::from(120), super::run(5));
  }
}
