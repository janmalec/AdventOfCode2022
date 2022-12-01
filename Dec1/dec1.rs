
use std::io::{self, BufRead};

fn main() {
    let mut biggest_deer = 0;
    let mut deer = 0;
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
      n1 = line.unwrap().parse::<i32>().unwrap();
      println!("Number {}", n1)
    }
  }