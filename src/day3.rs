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
struct  ParsedLine {
  line: [bool; 12],
}

struct BinaryPositionCounter {
  lines: std::io::Lines<BufReader<File>>,
}

impl BinaryPositionCounter {
  fn new(path: &str) -> BinaryPositionCounter {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    BinaryPositionCounter {
      lines,
    }
  }
}

impl Iterator for BinaryPositionCounter {
  type Item = Result<ParsedLine, DayTwoError>;
  fn next(&mut self) -> Option<Self::Item> {
    let line = match self.lines.next()? {
      Ok(line) => line,
      Err(err) => return Some(Err(err.into())),
    };

    let mut chars = line.chars();

    let parsed_line = ParsedLine { line: [
      chars.next()? == '1',
      chars.next()? == '1',
      chars.next()? == '1',
      chars.next()? == '1',
      chars.next()? == '1',
      chars.next()? == '1',
      chars.next()? == '1',
      chars.next()? == '1',
      chars.next()? == '1',
      chars.next()? == '1',
      chars.next()? == '1',
      chars.next()? == '1',
    ] };

    Some(Ok(parsed_line))
  }
}

pub fn main() -> Result<(), DayTwoError> {
  let mut counts = [0;12];
  for line in BinaryPositionCounter::new("./src/day-03.txt") {
    let line = line?.line;
    for i in 0..counts.len() {
      if line[i] {
        counts[i] +=1;
      } else {
        counts[i] -=1;
      }
    }
  }
  let result = counts.into_iter().rev().enumerate().fold((0,0), |acc, (i, x)| {
    let (gamma, epsilon) = acc;
    if x > 0 {
      (gamma + 2_i32.pow(i.try_into().unwrap()), epsilon)
    } else {
      (gamma, epsilon + 2_i32.pow(i.try_into().unwrap()))
    }
  });

  println!("{:?} {:?}, {:?}", counts, result, result.0 * result.1);
  Ok(())
}
