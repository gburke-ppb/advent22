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

// outer and inner are of the form number-number (e.g. 4-19)
fn contains(outer: &str, inner: &str) -> bool {
  let outer_str = String::from(outer);
  let mut outer_range = outer_str.split("-");
  let outer_lower = outer_range.next().unwrap().parse::<u32>().unwrap();
  let outer_upper = outer_range.next().unwrap().parse::<u32>().unwrap();

  let inner_str = String::from(inner);
  let mut inner_range = inner_str.split("-");
  let inner_lower = inner_range.next().unwrap().parse::<u32>().unwrap();
  let inner_upper = inner_range.next().unwrap().parse::<u32>().unwrap();

  inner_lower >= outer_lower && inner_upper <= outer_upper
}

fn part1(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));

  let mut count: u32 = 0;

  for input in infile.lines() {
    if let Ok(line) = input {
      let mut assignments = line.split(",");
      let elf_a = assignments.next().unwrap();
      let elf_b = assignments.next().unwrap();
      if contains(elf_a, elf_b) || contains(elf_b, elf_a) {
        count += 1;
      }
    }
  }
  println!("Part one: Total score = {count}");
}

fn part2(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));

  for input in infile.lines() {
    if let Ok(line) = input {}
  }
  println!("Part two: Total score = {}", 0);
}
