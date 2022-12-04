use std::io::{self, BufRead};
use std::collections::HashSet;

fn to_numbers(s: &str) -> Result<Vec<u8>, ()> {
  s.as_bytes().iter().map(|&x| {
    if x >= 97 && x < 123 {
      Ok(x - 96)
    } else if x < 97 && x >= 65  {
      Ok(x - 65 + 27)
    } else {
      Err(())
    }
  }).collect()
}

fn main() {
  let stdin = io::stdin();
  let mut result : u32 = 0;
  let mut result_second : u32 = 0;
  let mut three_buffer : Vec<HashSet<u8>> = Vec::new();
  for line in stdin.lock().lines() {
    let linestr = line.unwrap().to_string();
    let half_one = to_numbers(&linestr[0..linestr.len() / 2]).unwrap();
    let half_two = to_numbers(&linestr[linestr.len() / 2..linestr.len()]).unwrap();
    let one_set : HashSet<u8> = half_one.into_iter().collect();
    let two_set : HashSet<u8> = half_two.into_iter().collect();
    let common = one_set.intersection(&two_set);
    let common_v = common.collect::<Vec<_>>();
    result += **common_v.get(0).unwrap() as u32;
    three_buffer.push(to_numbers(&linestr).unwrap().into_iter().collect());
    if three_buffer.len() >= 3 {
      let mut three_common = three_buffer.get(0).unwrap() 
      & three_buffer.get(1).unwrap();
      three_common = &three_common 
      & three_buffer.get(2).unwrap();
      result_second += *three_common.iter().next().unwrap() as u32;
      three_buffer.clear();
    }
  }
  println!("{}", result);
  println!("{}", result_second);
}