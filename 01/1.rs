use std::io;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn main() -> Result<()> {
  let mut maxes = [0, 0, 0];
  let mut calories = 0;

  for input in io::stdin().lines() {
    if let Ok(line) = input {
      if line.eq("") {
        if calories > maxes[0] {
          maxes[2] = maxes[1];
          maxes[1] = maxes[0];
          maxes[0] = calories;
        } else if calories > maxes[1] {
          maxes[2] = maxes[1];
          maxes[1] = calories;
        } else if calories > maxes[2] {
          maxes[2] = calories;
        }
        calories = 0;
      } else {
        let x: i32 = line.trim().parse().expect("Input not an integer");
        calories += x;
      }
    }
  }
  println!(
    "{} + {} + {} = {}",
    maxes[0],
    maxes[1],
    maxes[2],
    maxes[0] + maxes[1] + maxes[2]
  );
  Ok(())
}
