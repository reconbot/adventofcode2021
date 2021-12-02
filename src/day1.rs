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

struct Int32SlidingWindow {
  values: Int32FileReader,
  last_three: [i32; 3],
  populated: bool,
}

impl Int32SlidingWindow {
  fn new(path: &str) -> Int32SlidingWindow {
    let values = Int32FileReader::new(path);
    Int32SlidingWindow {
      values,
      last_three: [0,0,0],
      populated: false,
    }
  }
}

impl Iterator for Int32SlidingWindow {
  type Item = i32;
  fn next(&mut self) -> Option<Self::Item> {
    if self.populated {
      self.last_three[0] = self.last_three[1];
      self.last_three[1] = self.last_three[2];
      self.last_three[2] = self.values.next()?;
    } else {
      self.last_three[0] = self.values.next()?;
      self.last_three[1] = self.values.next()?;
      self.last_three[2] = self.values.next()?;
      self.populated = true;
    }
    Some(self.last_three[0] + self.last_three[1] + self.last_three[2])
  }
}

#[derive(Debug)]
struct DirectionCounts {
  increased: i32,
  decreased: i32,
}


pub fn day1() -> Option<()> {
  let mut values = Int32SlidingWindow::new("./src/day-01.txt");
  let mut counter = DirectionCounts {
    increased: 0,
    decreased: 0,
  };
  let mut last_value = values.next()?;
  println!("{:?}", last_value);
  for value in values {
    if value > last_value {
      // println!("{:?} (increased)", value);
      counter.increased += 1;
    } else if value < last_value {
      // println!("{:?} (decreased)", value);
      counter.decreased += 1;
    }
    last_value = value;
  }
  println!("{:?}", counter);
  Some(())
}
