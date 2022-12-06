use std::{
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

fn test_chars(input: &[char]) -> bool {
  for i in 0..input.len() {
    for j in i + 1..input.len() {
      if input[i] == input[j] {
        return false;
      }
    }
  }
  true
}

fn find_marker(str: &String) -> u32 {
  let chars: Vec<char> = str.chars().collect();
  let mut result: u32 = 0;
  for i in 0..(str.len() - 4) {
    if test_chars(&chars[i..i + 4]) {
      result = i as u32 + 4;
      break;
    }
  }
  result
}

fn find_message(str: &String) -> u32 {
  let chars: Vec<char> = str.chars().collect();
  let mut result: u32 = 0;
  for i in 0..(str.len() - 14) {
    if test_chars(&chars[i..i + 14]) {
      result = i as u32 + 14;
      break;
    }
  }
  result
}

fn part1(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();
  let mut marker: u32 = 0;
  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      marker = find_marker(&line);
    }
  }

  println!("Part one: answer = {marker}");
}

fn part2(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();
  let mut message: u32 = 0;
  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      message = find_message(&line);
    }
  }

  println!("Part two: answer = {message}");
}
