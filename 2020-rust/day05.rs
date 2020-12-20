use std::fs;
use std::cmp;

static FREE: usize = 1;
static TAKEN: usize = 0;

fn main() {
  println!("EXAMPLE\n------------");
  part1(&vec!["FBFBBFFRLR".to_string()]);
  
  println!("\nINPUT\n------------");
  let input = load_input();
  let max = part1(&input);
  part2(&input, max);
}

fn part1(input: &Vec<String>) -> usize {
  let mut max: usize = 0;
  for spec in input {
    let seat_id = parse_seat_id(&spec);
    max = cmp::max(max, seat_id);
  }
  println!("Max value: {}", max);
  return max;
}

fn part2(input: &Vec<String>, max: usize) {
  let mut seats = vec![FREE; max + 1];
  for spec in input {
    let seat_id = parse_seat_id(&spec);
    seats[seat_id] = TAKEN;
  }
  for (index, state) in seats.iter().enumerate() {
    if *state == FREE {
      println!("Seat {} is free", index);
    }
  }
}

fn load_input() -> Vec<String> {
  return fs::read_to_string("05.txt")
    .unwrap()
    .lines()
    .map(|line| line.to_string())
    .collect();
}

fn parse_seat_id(spec: &str) -> usize {
  let (row_spec, seat_spec) = spec.split_at(7);
  let row = parse_row(row_spec);
  let seat = parse_seat(seat_spec);
  let seat_id = row * 8 + seat;
  // println!("{}: {}", spec, seat_id);
  return seat_id;
}

fn parse_row(row_spec: &str) -> usize {
  return binary_search(row_spec, 127, 'B');
}

fn parse_seat(seat_spec: &str) -> usize {
  return binary_search(seat_spec, 7, 'R');
}

fn binary_search(input: &str, max: usize, inc_char: char) -> usize {
  let mut cur_min: usize = 0;
  let mut cur_max: usize = max;
  for c in input.chars() {
    // println!("{} {} {}", c, cur_min, cur_max);
    let offset = (cur_max - cur_min) / 2 + 1;
    if c == inc_char {
      cur_min += offset;
    } else {
      cur_max -= offset;
    }
  }
  // println!("{} {}", cur_min, cur_max);
  return cur_min;
}
