use std::fs::File;
use std::io::{self, BufRead, BufReader};

struct Int32FileReader {
  lines: std::io::Lines<BufReader<File>>,
}

impl Int32FileReader {
  fn new(path: &str) -> Int32FileReader {
    let file = File::open(path).unwrap();
    let lines = io::BufReader::new(file).lines();

    Int32FileReader {
      lines,
    }
  }
}

impl Iterator for Int32FileReader {
  type Item = i32;
  fn next(&mut self) -> Option<Self::Item> {
    let line = self.lines.next()?.unwrap();
    Some(line.parse::<i32>().unwrap())
  }
}

#[derive(Debug)]
struct DirectionCounts {
  increased: i32,
  decreased: i32,
}



pub fn day1() -> Option<()> {
  let mut values = Int32FileReader::new("./src/day-01.txt");
  let mut counter = DirectionCounts {
    increased: 0,
    decreased: 0,
  };
  let mut last_value = values.next()?;
  println!("{:?}", last_value);
  for value in values {
    if value > last_value {
      println!("{:?} (increased)", value);
      counter.increased += 1;
    } else if value < last_value {
      println!("{:?} (decreased)", value);
      counter.decreased += 1;
    }
    last_value = value;
  }
  println!("{:?}", counter);
  Some(())
}
