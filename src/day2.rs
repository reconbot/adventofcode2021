use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::num::ParseIntError;


#[derive(Debug)]
pub enum DayTwoError {
  IoError(io::Error),
  ParseIntError(ParseIntError),
}

impl From<io::Error> for DayTwoError {
  fn from(error: io::Error) -> Self {
    DayTwoError::IoError(error)
  }
}

impl From<ParseIntError> for DayTwoError {
  fn from(error: ParseIntError) -> Self {
    DayTwoError::ParseIntError(error)
  }
}

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
  type Item = Result<Command, DayTwoError>;
  fn next(&mut self) -> Option<Self::Item> {
    let line = match self.lines.next()? {
      Ok(line) => line,
      Err(err) => return Some(Err(err.into())),
    };
    let mut parts = line.split(" ");
    let cmd_name = parts.next()?;
    let value = match parts.next()?.parse::<i32>() {
      Ok(val)=> val,
      Err(err) => return Some(Err(err.into())),
    };
    let cmd = match cmd_name {
      "forward" => Command::Forward(value),
      "down" => Command::Down(value),
      "up" => Command::Up(value),
      _ => panic!("Unknown command {}", cmd_name),
    };

    Some(Ok(cmd))
  }
}

pub fn main() -> Result<(), DayTwoError> {
  let mut depth = 0;
  let mut horizontal= 0;
  let mut aim = 0;
  for cmd in CommandFileReader::new("./src/day-02.txt") {
    match cmd? {
      Command::Up(value) => aim -= value,
      Command::Down(value) => aim += value,
      Command::Forward(value) => {
        horizontal += value;
        depth += aim * value;
      },
    };
  }
  println!("Depth {}, Distance {}, Product {}", depth, horizontal, depth * horizontal);
  Ok(())
}
