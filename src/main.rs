fn readlines_i32(file: &str) -> Vec<i32> {
	let mut entries = vec![];
	for line in file.lines() {
		entries.push(line.parse::<i32>().unwrap());
	}

	entries
}

#[derive(Debug)]
struct DirectionCounts {
  increased: i32,
  decreased: i32,
}

fn day1() {
	let file = include_str!("./day-01.txt");
	let mut values = readlines_i32(file);
  let mut counter = DirectionCounts {
    increased: 0,
    decreased: 0,
  };
  let mut last_value = values.remove(0);
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
}

fn main() {
  day1();
}
