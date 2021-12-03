use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
enum Command {
  Forward(i32),
  Down(i32),
  Up(i32),
}
struct CommandFileReader {
  lines: std::io::Lines<BufReader<File>>,
}

impl CommandFileReader {
  fn new(path: &str) -> CommandFileReader {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    CommandFileReader {
      lines,
    }
  }
}

impl Iterator for CommandFileReader {
  type Item = Command;
  fn next(&mut self) -> Option<Self::Item> {
    let line = self.lines.next()?.unwrap();
    let mut parts = line.split(" ");
    let cmd_name = parts.next()?;
    let value = parts.next()?.parse::<i32>().unwrap();
    let cmd = match cmd_name {
      "forward" => Command::Forward(value),
      "down" => Command::Down(value),
      "up" => Command::Up(value),
      _ => panic!("Unknown command {}", cmd_name),
    };

    Some(cmd)
  }
}

pub fn day2() -> Option<()> {
  let mut depth = 0;
  let mut horizontal= 0;
  let mut aim = 0;
  for cmd in CommandFileReader::new("./src/day-02.txt") {
    match cmd {
      Command::Up(value) => aim -= value,
      Command::Down(value) => aim += value,
      Command::Forward(value) => {
        horizontal += value;
        depth += aim * value;
      },
    };
  }
  println!("Depth {}, Distance {}, Product {}", depth, horizontal, depth * horizontal);
  Some(())
}
