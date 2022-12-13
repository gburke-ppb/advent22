use num::{BigInt, Zero, Integer};
use std::{
  env,
  fs::File,
  io::{BufRead, BufReader},
  ops::{Add, Mul},
};

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  part1(&filename);
  part2(&filename);
}

#[derive(Debug)]
enum OperationType {
  Multiply,
  Add,
}

fn run_simulation(filename: &String, divisor: usize, rounds: usize) -> usize {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();

  let mut monkey: usize = 0;
  let mut counter: Vec<usize> = Vec::new();
  let mut items: Vec<Vec<BigInt>> = Vec::new();
  let mut operations: Vec<OperationType> = Vec::new();
  let mut operation_nums: Vec<BigInt> = Vec::new();
  let mut operations_val_is_old: Vec<bool> = Vec::new();
  let mut test_val: Vec<BigInt> = Vec::new();
  let mut if_true: Vec<usize> = Vec::new();
  let mut if_false: Vec<usize> = Vec::new();

  while let Some(input) = lines.next() {
    if let Ok(line) = input {
      if line.contains("Monkey") {
        let mut bits = line.trim().split(" ");
        bits.next(); // Starting
        let mut num_bits = bits.next().expect("whatever").split(":");
        monkey = num_bits.next().expect("whatever").parse().unwrap();
        items.push(Vec::new());
        counter.push(0);
      }
      if line.contains("Starting") {
        let mut bits = line.trim().split(" ");
        bits.next(); // Starting
        bits.next(); // items:
        while let Some(mut val) = bits.next() {
          let tmp = val.replace(",", "");
          val = tmp.as_str();
          let i: BigInt = val.parse().unwrap();
          items[monkey].push(i);
        }
      }
      if line.contains("Operation") {
        let mut bits = line.trim().trim().split(" ");
        bits.next(); // Operation:
        bits.next(); // new
        bits.next(); // =
        bits.next(); // old
        let op = bits.next().expect("whatever");
        let val = bits.next().expect("whatever");
        if op == "*" {
          operations.push(OperationType::Multiply);
        } else {
          operations.push(OperationType::Add);
        }
        if val == "old" {
          operations_val_is_old.push(true);
          operation_nums.push(BigInt::zero());
        } else {
          operations_val_is_old.push(false);
          operation_nums.push(val.parse().unwrap());
        }
      }
      if line.contains("Test") {
        let mut bits = line.trim().split(" ");
        bits.next(); // Test:
        bits.next(); // divisible
        bits.next(); // by
        let val: BigInt = bits.next().expect("whatever").parse().unwrap();
        test_val.push(val);
      }
      if line.contains("If true") {
        let mut bits = line.trim().split(" ");
        bits.next(); // If
        bits.next(); // true:
        bits.next(); // throw
        bits.next(); // to
        bits.next(); // monkey
        let val: usize = bits.next().expect("whatever").parse().unwrap();
        if_true.push(val);
      }
      if line.contains("If false") {
        let mut bits = line.trim().split(" ");
        bits.next(); // If
        bits.next(); // false:
        bits.next(); // throw
        bits.next(); // to
        bits.next(); // monkey
        let val: usize = bits.next().expect("whatever").parse().unwrap();
        if_false.push(val);
      }
    }
  }

  for _ in 0..rounds {
    for j in 0..monkey + 1 {
      for x in 0..items[j].len() {
        let mut current = items[j][x].clone();
        counter[j] += 1;
        match operations[j] {
          OperationType::Add => {
            if operations_val_is_old[j] {
              let tmp = current.clone().add(current);
              current = BigInt::from(tmp);
            } else {
              let tmp = current.add(&operation_nums[j]);
              current = BigInt::from(tmp);
            }
          }
          OperationType::Multiply => {
            if operations_val_is_old[j] {
              let tmp = current.clone().mul(current);
              current = BigInt::from(tmp);
            } else {
              let tmp = current.mul(&operation_nums[j]);
              current = BigInt::from(tmp);
            }
          }
        }
        let tmp=current.div_floor(&BigInt::from(divisor));
        current = BigInt::from(tmp);
        if &current % &test_val[j] == BigInt::zero() {
          items[if_true[j]].push(current);
        } else {
          items[if_false[j]].push(current);
        }
      }
      items[j] = Vec::new();
    }
    // println!("{:?}", counter);
    // println!("{:?}", items);
    // println!("");
  }

  counter.sort_by(|a, b| b.cmp(a));
  let x = counter[0];
  let y = counter[1];

  x * y
}

fn part1(filename: &String) {
  println!("Part one: answer = {}", run_simulation(filename, 3, 20));
}
fn part2(filename: &String) {
  println!("Part two: answer = {}", run_simulation(filename, 1, 10000));
}
