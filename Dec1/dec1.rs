
use std::io::{self, BufRead};

fn main() {
    let mut deer = 0;
    let mut deer_weights = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
      let linestr = line.unwrap().to_string();
      if linestr.len() == 0 {
        deer_weights.push(deer);
        deer = 0;
      } else {
        deer += linestr.parse::<i32>().unwrap();
      } 
    }
    deer_weights.sort();
    let len = deer_weights.len();
    println!("Deer with most weight {}", deer_weights[len-1]);
    let top_3_deer : Vec<i32> = deer_weights.drain(len-3..).collect();
    let top_sum : i32 = top_3_deer.iter().sum();
    println!("Top 3: {}", top_sum);
  }