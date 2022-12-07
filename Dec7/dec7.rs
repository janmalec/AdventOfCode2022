use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();
  let mut path = Vec::new();
  path.push("/".to_string());
  for line in stdin.lock().lines() {
    let linestr = line.unwrap().to_string();
    let split: Vec<String> = linestr.split_whitespace().map(str::to_string).collect();
    println!("{:?}", split);
  }
}