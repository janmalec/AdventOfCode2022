use std::io::{self, BufRead, Write};
use std::{thread, time};

fn print_boxes(a: &[Vec<char>]) {
  // Find out how tall is the tallest stack
  let mut max_len = 0;
  for i in 0..a.len() {
    if a[i].len() > max_len {
      max_len = a[i].len();
    }
  }
  // Print them crates
  for i in 0..max_len {
    for v in a {
      if v.len() >= max_len-i {
        print!("[{}] ", v.get(max_len-i-1).unwrap());
      } else {
        print!("    ");
      }
    }
    print!("\n");
    io::stdout().flush().unwrap();
  }
  println!(" 1   2   3   4   5   6   7   8   9 ");
}

fn main() {
  let millis = time::Duration::from_millis(20);
  let stdin = io::stdin();
  //let mut result : u32 = 0;
  let mut array: [Vec<char>; 9] = Default::default();
  for line in stdin.lock().lines() {
    let linestr = line.unwrap().to_string();
    if linestr.len() == 0 || linestr.chars().nth(1).unwrap() == '1' {
      break;
    }
    for i in 0..array.len() {
      let current_box = linestr.chars().nth(1+4*i).unwrap();
      if current_box != ' ' {
        array[i].push(current_box);
      }
    }
  }
  for i in 0..array.len() {
    array[i].reverse();
  }

  print_boxes(&array);
  // Skip the empty line
  stdin.lock().lines().next();
  for line in stdin.lock().lines() {
    let linestr = line.unwrap().to_string();
    let split: Vec<&str> = linestr.split_whitespace().collect();
    if split.len() == 0 {
      break;
    }
    let num :usize  = split.get(1).unwrap().parse().unwrap();
    let from :usize = split.get(3).unwrap().parse().unwrap();
    let to :usize   = split.get(5).unwrap().parse().unwrap();
    println!("{} from {} to {}", num, from, to);
    for _i in 0..num {
      let tmp = array[from-1].pop().unwrap();
      array[to-1].push(tmp);
    }
    thread::sleep(millis);
    print!("{}[2J", 27 as char);
    print_boxes(&array);
  }
}