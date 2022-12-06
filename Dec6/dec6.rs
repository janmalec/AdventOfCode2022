use std::collections::HashSet;
use std::io::{self, BufRead};


// I made the GPT3 AI write this function :)
fn has_duplicates(v: &Vec<char>) -> bool {
  // Create a new HashSet
  let mut seen = HashSet::new();

  // Iterate over the elements in the vector
  for elem in v {
      // If the element is already in the set, then it is a duplicate
      if seen.contains(&elem) {
          return true;
      }

      // Otherwise, add the element to the set
      seen.insert(elem);
  }

  // If we reach here, then there are no duplicates in the vector
  false
}

fn main() {
  let stdin = io::stdin();
  let input = stdin.lock().lines().next().unwrap().unwrap();
  println!("{}", input);
  let mut i = 0;
  while i+14 < input.len() {
    if !has_duplicates(&input[i..i+14].chars().collect()) {
      println!("{} {:?}", i+14, &input[i..i+14]);
      break;
    }
    i += 1;
  }
}