use std::io;
use std::io::prelude::*;

pub fn main() {
    let reader = io::stdin();
    let total: u32 = readline(&reader).parse().unwrap();
    for _ in 0..total {
        let s = readline(&reader);
        let n: u32 = s.parse().unwrap();
        let digits = s
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        println!("{}", run(n, digits));
    }
}

fn run(n: u32, digits: Vec<u32>) -> u32 {
    let mut count = 0;
    for digit in digits {
        if digit != 0 && n % digit == 0 {
            count += 1;
        }
    }
    count
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
        assert_eq!(3, super::run(111, vec![1, 1, 1]));
        assert_eq!(3, super::run(1012, vec![1, 0, 1, 2]));
    }
}
