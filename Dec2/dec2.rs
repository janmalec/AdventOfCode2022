use std::io::{self, BufRead};
use std::collections::HashMap;


fn main() {

  let equal = HashMap::from([
    ('A', 'X'),
    ('B', 'Y'),
    ('C', 'Z'),
  ]);
  
  let wins_to = HashMap::from([
    ('A', 'Z'),
    ('B', 'X'),
    ('C', 'Y'),
  ]);
  
  let looses_to = HashMap::from([
    ('A', 'Y'),
    ('B', 'Z'),
    ('C', 'X'),
  ]);

  let scores = HashMap::from([
    ('X', 1),
    ('Y', 2),
    ('Z', 3),
  ]);
 
  /*
  let names = HashMap::from([
    ('A', "rock".to_string()),
    ('B', "paper".to_string()),
    ('C', "scissors".to_string()),
    ('X', "rock".to_string()),
    ('Y', "paper".to_string()),
    ('Z', "scissors".to_string()),
  ]);
 */
  let mut score = 0;
  let mut score_two = 0;
  let stdin = io::stdin();
  for line in stdin.lock().lines() {
    let linestr = line.unwrap().to_string();
    let first = linestr.chars().nth(0).unwrap();
    let second = linestr.chars().nth(2).unwrap();
    let mypick : char;
    
    // points based on score
    // if tied
    if equal.get(&first).unwrap() == &second {
      score += 3;
      // if lost
    } else if wins_to.get(&first).unwrap() == &second {
      score += 0;
      // if won
    } else {
      score += 6;
    }
    score += scores.get(&second).unwrap();

    // Second part
    if second == 'Y' {
      mypick = *equal.get(&first).unwrap();
      score_two += 3;
    } else if second == 'X' {
      mypick = *wins_to.get(&first).unwrap();
      score_two += 0;
    } else{
      mypick = *looses_to.get(&first).unwrap();
      score_two += 6;
    }
    score_two += scores.get(&mypick).unwrap();

  }
  println!("{}", score);
  println!("{}", score_two);
}