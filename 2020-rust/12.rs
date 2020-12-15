use core::panic;
use std::{fmt::Display, fs};

fn main() {
  let input = read_input("12-example.txt");
  println!("EXAMPLE\n------------");
  part1(&input);
  part2(&input);

  let input = read_input("12.txt");
  println!("\nINPUT\n------------");
  part1(&input);
  part2(&input);
}

fn part1(input: &Vec<Command>) {
  let mut boat = BoatPt1::new();
  for command in input {
    match command.letter {
      CommandLetter::N => boat.y += command.value,
      CommandLetter::E => boat.x += command.value,
      CommandLetter::S => boat.y -= command.value,
      CommandLetter::W => boat.x -= command.value,
      CommandLetter::L => boat.rot += command.value,
      CommandLetter::R => boat.rot -= command.value,
      CommandLetter::F => {
        let dir = (rad(boat.rot).cos(), rad(boat.rot).sin());
        boat.x += command.value * dir.0.round() as i64;
        boat.y += command.value * dir.1.round() as i64;
      }
    }
  }
  println!("{}", boat);
}

fn rad(input: i64) -> f64 {
  (input as f64).to_radians()
}

struct BoatPt1 {
  x: i64,
  y: i64,
  rot: i64
}
impl BoatPt1 {
  fn new() -> BoatPt1 {
    BoatPt1 { x: 0, y: 0, rot: 0 }
  }
}
impl Display for BoatPt1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Boat[x={},y={},rot={},manhattan={}]", self.x, self.y, self.rot, self.x.abs() + self.y.abs())
    }
}

fn part2(input: &Vec<Command>) {
  let mut boat = BoatPt2::new();
  for command in input {
    match command.letter {
      CommandLetter::N => boat.wy += command.value,
      CommandLetter::E => boat.wx += command.value,
      CommandLetter::S => boat.wy -= command.value,
      CommandLetter::W => boat.wx -= command.value,
      CommandLetter::L => {
        let buf = boat.wx;
        boat.wx = -boat.wy;
        boat.wy = buf;
      },
      CommandLetter::R => {
        let buf = boat.wx;
        boat.wx = boat.wy;
        boat.wy = -buf;
      },
      CommandLetter::F => {
        boat.x += command.value * boat.wx;
        boat.y += command.value * boat.wy;
      }
    }
  }
  println!("{}", boat); // not 13907
}

struct BoatPt2 {
  x: i64,
  y: i64,
  wx: i64,
  wy: i64
}
impl BoatPt2 {
  fn new() -> BoatPt2 {
    BoatPt2 { x: 0, y: 0, wx: 10, wy: 1 }
  }
}
impl Display for BoatPt2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Boat[x={},y={},wx={},wy={},manhattan={}]", self.x, self.y, self.wx, self.wy, self.x.abs() + self.y.abs())
    }
}

enum CommandLetter {
  N, E, S, W, L, R, F
}

struct Command {
  letter: CommandLetter,
  value: i64
}

fn read_input(file_name: &str) -> Vec<Command> {
  fs::read_to_string(file_name).unwrap()
    .lines()
    .map(|l| l.split_at(1))
    .map(|(c, value)| Command {
      letter: match c.chars().next().unwrap() {
        'N' => CommandLetter::N,
        'E' => CommandLetter::E,
        'S' => CommandLetter::S,
        'W' => CommandLetter::W,
        'L' => CommandLetter::L,
        'R' => CommandLetter::R,
        'F' => CommandLetter::F,
        _ => panic!("unknown command letter"),
      },
      value: value.parse::<i64>().unwrap()
    })
    .collect()
}
