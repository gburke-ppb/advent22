use std::{
  env,
  fs::File,
  io::{BufRead, BufReader},
  ops::Add,
};

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  part1(&filename);
  part2(&filename);
}

// width is (3*n)+(n-1) characters
// => num stacks = (num chars + 1) / 4
// [J]             [B] [W]
// [T]     [W] [F] [R] [Z]
// [Q] [M]     [J] [R] [W] [H]
// [F] [L] [P]     [R] [N] [Z] [G]
// [F] [M] [S] [Q]     [M] [P] [S] [C]
// [L] [V] [R] [V] [W] [P] [C] [P] [J]
// [M] [Z] [V] [S] [S] [V] [Q] [H] [M]
// [W] [B] [H] [F] [L] [F] [J] [V] [B]
//  1   2   3   4   5   6   7   8   9

// move 3 from 5 to 7
// move 2 from 8 to 9
// move 4 from 3 to 5
// move 2 from 1 to 7
// move 1 from 3 to 6
// move 2 from 1 to 7
// move 1 from 8 to 7
// move 4 from 2 to 8

fn parse_stacks(line: &String) -> (Vec<char>, usize) {
  let len = line.len();
  let chars: Vec<_> = line.chars().collect();

  let num_cols = (len + 1) / 4;

  let mut cols: Vec<char> = Vec::with_capacity(num_cols);

  // println!("We have {num_cols} stacks");

  let mut count = 0;

  for i in 0..num_cols {
    let num = 1 + (i * 4);
    // println!("col {i} is a '{}'", chars[num]);
    if !chars[num].is_numeric() {
      cols.push(chars[num]);
      count += 1;
    }
  }

  (cols, count)
}

fn parse_instructions(line: &String) -> (usize, usize, usize) {
  let parts: Vec<_> = line.split(" ").collect();

  (
    parts[1].parse().unwrap(),
    parts[3].parse().unwrap(),
    parts[5].parse().unwrap(),
  )
}

fn part1(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));

  let mut stacks: Vec<Vec<char>> = Vec::new();

  let mut lines = infile.lines();

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      // println!(">{line}<");
      if line == "" {
        break;
      }
      let (crates, num) = parse_stacks(&line);
      // println!("Received {num} crates");
      if stacks.is_empty() {
        for _ in 0..num {
          stacks.push(Vec::new());
        }
      }
      for i in 0..num {
        // println!("{num}");
        if crates[i] != ' ' {
          stacks[i].push(crates[i]);
        }
      }
    }
  }

  for i in 0..stacks.len() {
    stacks[i].reverse();
  }

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      // println!("|>{line}<|");
      let (num, from, to) = parse_instructions(&line);
      for i in 0..num {
        let block = stacks[from - 1].pop().unwrap();
        stacks[to - 1].push(block)
      }
    }
  }

  print!("Part one: answer = ");
  for i in 0..stacks.len() {
    print!("{}", stacks[i].pop().unwrap());
  }
  println!("");
}

fn part2(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));

  let mut stacks: Vec<Vec<char>> = Vec::new();

  let mut lines = infile.lines();

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      // println!(">{line}<");
      if line == "" {
        break;
      }
      let (crates, num) = parse_stacks(&line);
      // println!("Received {num} crates");
      if stacks.is_empty() {
        for _ in 0..num {
          stacks.push(Vec::new());
        }
      }
      for i in 0..num {
        // println!("{num}");
        if crates[i] != ' ' {
          stacks[i].push(crates[i]);
        }
      }
    }
  }

  for i in 0..stacks.len() {
    stacks[i].reverse();
  }

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      // println!("|>{line}<|");
      let (num, from, to) = parse_instructions(&line);
      let mut blocks: Vec<char> = Vec::new();
      for i in 0..num {
        let block = stacks[from - 1].pop().unwrap();
        blocks.push(block);
      }
      for i in 0..num {
        let block = blocks.pop().unwrap();
        stacks[to - 1].push(block);
      }
    }
  }

  print!("Part two: answer = ");
  for i in 0..stacks.len() {
    print!("{}", stacks[i].pop().unwrap());
  }
  println!("");
}
