use std::io::{self, BufRead};

fn check_tree(i: usize, j: usize, map: & Vec<Vec<u32>>) -> bool {
  let mut flag = false;
  for k in 0..i {
    if map[k][j] >= map[i][j] {
      flag = true;
      break;
    }
  }
  if flag == false{
    return flag;
  }
  flag = false;
  for k in i+1..map.len() {
    if map[k][j] >= map[i][j] {
      flag = true;
      break;
    }
  }
  if flag == false{
    return flag;
  }
  flag = false;
  for k in 0..j {
    if map[i][k] >= map[i][j] {
      flag = true;
      break;
    }
  }
  if flag == false{
    return flag;
  }
  flag = false;
  for k in j+1..map[0].len() {
    if map[i][k] >= map[i][j] {
      flag = true;
      break;
    }
  }
  if flag == false{
    return flag;
  }
  true
}

fn tree_view(i: usize, j: usize, map: & Vec<Vec<u32>>) -> u32 {
  let mut view_l = 0;
  let mut view_r = 0;
  let mut view_u = 0;
  let mut view_d = 0;
  for k in (0..j).rev() {
    view_l += 1;
    if map[i][k] >= map[i][j] {
      break;
    }
  }
  for k in j+1..map[0].len() {
    view_r += 1;
    if map[i][k] >= map[i][j] {
      break;
    }
  }
  for k in (0..i).rev() {
    view_u += 1;
    if map[k][j] >= map[i][j] {
      break;
    }
  }
  for k in i+1..map.len() {
    view_d += 1;
    if map[k][j] >= map[i][j] {
      break;
    }
  }
  view_l*view_r*view_u*view_d
}

fn main() {
  let stdin = io::stdin();
  let mut map : Vec<Vec<u32>> = Vec::new();
  for line in stdin.lock().lines() {
    let linevec: Vec<u32> = line.unwrap()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    map.push(linevec);
  }
  println!("{:?}", map);
  let mut visible = map.len()*map[0].len();
  let mut max_view: u32 = 0;
  for i in 1..map.len()-1 {
    for j in 1..map[0].len()-1 {
      //println!("{} {} {} {}", i, j, map[i][j], check_tree(i, j, &map));
      if check_tree(i, j, &map) {
        visible -= 1;
      }
      let view = tree_view(i, j, &map);
      if view > max_view {
        max_view = view;
      }
    }
  }
  println!("Visible trees: {}", visible);
  println!("Max view: {}", max_view);
  println!("view at 1 2 : {}", tree_view(1, 2, &map));

}