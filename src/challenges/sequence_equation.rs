use std::io;
use std::io::prelude::*;

pub fn main() {
  let reader = io::stdin();
  let n: usize = readline(&reader).parse().unwrap();
  let v = readline(&reader)
    .split(" ")
    .map(|x| x.parse().unwrap())
    .collect::<Vec<usize>>();
  for i in 1..(n + 1) {
    println!("{}", run(&v, i));
  }
}

fn run(v: &Vec<usize>, i: usize) -> usize {
  for j in 0..v.len() {
    if v[v[j] - 1] == i {
      return j + 1;
    }
  }
  return 0;
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
  #[test]
  fn it_works() {
    let v = vec![2, 3, 1];
    assert_eq!(2, super::run(&v, 1));
    assert_eq!(3, super::run(&v, 2));
    assert_eq!(1, super::run(&v, 3));
  }
}
