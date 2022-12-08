use std::{
  env,
  fs::File,
  io::{BufRead, BufReader},
};

enum InodeType {
  Dir,
  File,
}

struct Inode {
  name: String,
  itype: InodeType, // `type` is a reserved word
  size: u64,
  children: Vec<Inode>,
}

impl Inode {
  fn add_dir(&mut self, dirname: &String) {
    let node = Inode {
      name: String::from(dirname),
      itype: InodeType::Dir,
      size: 0,
      children: Vec::new(),
    };
    self.children.push(node);
  }

  fn add_file(&mut self, filename: &str, size: u64) {
    let node = Inode {
      name: String::from(filename),
      itype: InodeType::File,
      size: size,
      children: Vec::new(),
    };
    self.children.push(node);
    self.size += size;
  }
}

pub fn main() {
  let args: Vec<String> = env::args().collect();
  let filename = &args[1];

  part1(&filename);
  part2(&filename);
}

fn part1(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();

  let mut fs: Inode = Inode {
    name: String::from("/"),
    itype: InodeType::Dir,
    size: 0, // if a file, the size of the file.  If a dir, the size of all files and subdirs
    children: Vec::new(),
  };

  while let Some(input) = lines.next() {
    if let Ok(mut line) = input {
      if line.starts_with("$ ") {
        let mut command = line.split_off(2);
        if command.starts_with("cd ") {
          let dirname = command.split_off(3);
          if dirname == ".." {

          } else if dirname == "/" {

          } else {

          }
        } else if command.starts_with("ls") {
          continue;
        }

      } else if line.starts_with("dir") {
        //
        let dirname = line.split_off(4);
        // something.add_dir(&dirname);
      } else {
        // it starts with a number.
        let strings: Vec<_> = line.split(" ").collect();
        let str_size = strings[0];
        let filename = strings[1];
        let size: usize = str_size.parse().unwrap();

        // something.add_file(&filename, size);
      }
    }
  }

  print!("Part one: answer = ");
}

fn part2(filename: &String) {
  let infile = BufReader::new(File::open(filename).expect("Can't open that file"));
  let mut lines = infile.lines();
  while let Some(input) = lines.next() {
    if let Ok(line) = input {}
  }

  print!("Part two: answer = ");
}
