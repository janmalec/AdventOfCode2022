use std::io::{self, BufRead};

fn main() {
  let stdin = io::stdin();
  let mut result : u32 = 0;
  let mut result_two : u32 = 0;
  for line in stdin.lock().lines() {
    let linestr = line.unwrap().to_string();
    let split: Vec<&str> = linestr.split(&['-', ','][..]).collect();
    let ranges: Vec<u32> = split.iter().map(|s| s.parse().unwrap()).collect();
    // They completely overlap one lower bound is higher or same as other and 
    // higher bound lower or same as other
    if ( ranges.get(0) >= ranges.get(2) && ranges.get(1) <= ranges.get(3) ) 
    || ( ranges.get(2) >= ranges.get(0) && ranges.get(3) <= ranges.get(1) ) {
      result += 1;
    }
    // They overlap at all if one higher or same bound is lower than other lower bound
    if ( ranges.get(1) >= ranges.get(2) ) && ( ranges.get(1) <= ranges.get(3) )
    || ( ranges.get(3) >= ranges.get(0) ) && ( ranges.get(3) <= ranges.get(1) ) {
      result_two += 1;
    }
  }
  println!("{}", result);
  println!("{}", result_two);
}