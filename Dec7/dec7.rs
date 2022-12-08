use std::io::{self, BufRead};

struct directory {
  name: String,
  
}

fn main() {
  let stdin = io::stdin();
  let mut path : Vec<String> = Vec::new();
  //path.push("/".to_string());
  for line in stdin.lock().lines() {
    let linestr = line.unwrap().to_string();
    let split: Vec<String> = linestr.split_whitespace().map(str::to_string).collect();
    println!("{:?} {:?}", split, path);
    if split.get(0).unwrap() == &"$" {
      if split.get(1).unwrap() == &"cd".to_string() {
        if split.get(2).unwrap() == &"..".to_string() {
          path.pop();
        } else {
          path.push(split.get(2).unwrap().to_string());
        }
      }
    }
  }
}