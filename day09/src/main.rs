use std::{
  collections::HashMap,
  env,
  fs::File,
  io::{BufRead, BufReader},
};

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  part1(&filename);
  part2(&filename);
}

fn test_move_tail(head: (i32, i32), tail: (i32, i32)) -> bool {
  if head.0.abs_diff(tail.0) > 1 || head.1.abs_diff(tail.1) > 1 {
    return true;
  }
  false
}

fn part1(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();

  let mut h_now: (i32, i32) = (0, 0); // (x,y)
  let mut h_last: (i32, i32) = (0, 0);
  let mut t: (i32, i32) = (0, 0);
  let mut grid: HashMap<(i32, i32), bool> = HashMap::new();
  grid.insert(t, true);

  while let Some(input) = lines.next() {
    if let Ok(mut line) = input {
      let v: Vec<&str> = line.split(" ").collect();
      let dir = v[0];
      let num: i32 = v[1].parse().unwrap();
      for i in 0..num {
        match dir {
          "U" => {
            h_now = (h_now.0, h_now.1 + 1);
          }
          "D" => {
            // y--
            h_now = (h_now.0, h_now.1 - 1);
          }
          "R" => {
            // x++
            h_now = (h_now.0 + 1, h_now.1);
          }
          "L" => {
            // y--
            h_now = (h_now.0 - 1, h_now.1);
          }
          s => {
            println!("Unknown movement {s}");
          }
        }
        if test_move_tail(h_now, t) {
          t = h_last;
          grid.insert(t, true);
        }
        h_last = h_now;
      }
    }
  }

  // println!("tail visited");
  // for c in &grid {
  //   println!("({},{})", c.0.0, c.0.1);
  // }
  println!("Part one: answer = {}", grid.len());
}

fn part2(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();
  while let Some(input) = lines.next() {
    if let Ok(line) = input {}
  }

  print!("Part two: answer = ");
}
