use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
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

#[derive(Debug, Clone)]
struct BingoBoard {
    numbers: [i32; 25],
    state: [bool; 25],
}

impl BingoBoard {
    fn new(numbers: [i32; 25]) -> BingoBoard {
        BingoBoard {
            numbers,
            state: [false; 25],
        }
    }

    fn play(&mut self, number: i32) -> Option<i32> {
        for (position, board_number) in self.numbers.iter().enumerate() {
            if *board_number == number {
                self.state[position] = true;
            }
        }
        Some(number * self.check_for_win()?)
    }

    fn sum_unmarked_numbers(&self) -> i32 {
      let mut sum = 0;
      for (i, marked) in self.state.into_iter().enumerate() {
        if !marked {
          sum = sum + self.numbers[i];
        }
      }

      sum
    }

    fn check_for_win(&self) -> Option<i32> {
        let state = self.state;
        for index in 0..4 {
            if state[index]
                && state[index + 5]
                && state[index + 10]
                && state[index + 15]
                && state[index + 20]
            {
                return Some(self.sum_unmarked_numbers());
            }
            if state[index * 5]
                && state[index * 5 + 1]
                && state[index * 5 + 2]
                && state[index * 5 + 3]
                && state[index * 5 + 4]
            {
              return Some(self.sum_unmarked_numbers());
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_marks_numbers_used() {
        let mut board = BingoBoard::new([
            0, 0, 1, 1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 1, 0, 0,
        ]);
        assert_eq!(board.play(0), None);
        assert_eq!(board.play(1), None);
        assert_eq!(
            board.state,
            [
                true, true, true, true, false, false, false, false, false, false, false, false,
                false, false, false, false, false, false, false, false, false, false, true, true,
                true
            ]
        );
    }

    #[test]
    fn it_detects_horizontal_wins() {
        let mut board = BingoBoard::new([
            14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3,
            7,
        ]);
        assert_eq!(board.play(14), None);
        assert_eq!(board.play(21), None);
        assert_eq!(board.play(17), None);
        assert_eq!(board.play(4), None);
        assert_eq!(board.play(24), Some(5880));
        assert_eq!(board.check_for_win(), Some(245));
    }

    #[test]
    fn it_detects_vertical_wins() {
        let mut board = BingoBoard::new([
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24,
        ]);
        assert_eq!(board.play(1), None);
        assert_eq!(board.play(6), None);
        assert_eq!(board.play(11), None);
        assert_eq!(board.play(16), None);
        assert_eq!(board.play(21), Some(5145));
        assert_eq!(board.check_for_win(), Some(245));
    }
}

#[derive(Debug)]
struct Game {
    boards: Vec<BingoBoard>,
    moves: Vec<i32>,
}

impl Game {
    fn new(path: &str) -> Game {
        let file = File::open(path).unwrap();
        let lines = io::BufReader::new(file).lines().map(|x| x.unwrap());
        Game::new_from_lines(lines)
    }

    fn new_from_lines<T: Iterator<Item = String>>(mut lines: T) -> Game {
      let moves = lines
          .next()
          .unwrap()
          .split(",")
          .map(|x| x.parse::<i32>().unwrap())
          .collect();
      let re = Regex::new(r"\s+").unwrap();
      let mut boards = vec![];
      loop {
          // skip blanks and end parsing
          if lines.next().is_none() {
              break;
          }

          let mut board_input: [i32; 25] = [0; 25];
          let board_string: Vec<i32> = (0..5)
              .flat_map(|_| {
                  let line = lines.next().unwrap();
                  let line = line.trim();

                  re.split(line)
                      .map(|x| match x.parse::<i32>() {
                          Ok(value) => value,
                          Err(e) => {
                              println!("{:?} {:?} {:?}", x, e, line);
                              panic!("ParseIntError");
                          }
                      })
                      .collect::<Vec<i32>>()
              })
              .collect();

          for index in 0..24 {
              board_input[index] = board_string[index];
          }

          boards.push(BingoBoard::new(board_input));
      }
      Game { boards, moves }
  }

    fn play(&mut self) -> Option<(BingoBoard, i32)> {
        for num in self.moves.as_slice() {
            for (id, board) in self.boards.iter_mut().enumerate() {
                if board.check_for_win().is_some() {
                    continue;
                }
                println!("playing {} on board {}", num, id);
                match board.play(*num) {
                    Some(product) => {
                        let sum = board.check_for_win().unwrap_or(-1);
                        println!("{:?}", board);
                        println!("Board {} won with {}, a sum of {} and a product of {}!", id, num, sum, product);
                        // return Some((board.clone(), product));
                    },
                    None => {}
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod game_tests {
  use super::*;

  #[test]
  fn sample_game() {
    let input = include_str!("day-04-sample.txt");

    let lines = input.lines().map(|s| s.to_string());
    let mut game = Game::new_from_lines(lines);
    let winner = game.play();
    match winner {
        Some((_bingo, product)) => assert_eq!(product, 4512),
        None => panic!("if only"),
    }
  }
}

pub fn main() -> Result<(), DayTwoError> {
  let mut game = Game::new("./src/day-04.txt");
  let winner = game.play();
  println!("{:?}", winner);
  Ok(())
}
