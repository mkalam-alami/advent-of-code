#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use regex::Regex;

lazy_static! {
  static ref COMMENT_REGEX: Regex = Regex::new("[a-z:]+$").unwrap();
  static ref ALLOWED_RANGES_REGEX: Regex = Regex::new("([^:]+): ([0-9-]+) or ([0-9-]+)").unwrap();
}

fn main() {
  let input = read_input("16-example.txt");
  println!("EXAMPLE\n----------");
  part1(&input);
}

fn part1(input: &Input) {
  println!("rules={},your={},nearby={}", input.allowed_ranges.keys().len(), input.your_ticket.len(), input.nearby_tickets.len());
  println!("RESULT: TODO");
}

struct Range(i64, i64);

type Ticket = Vec<i64>;

struct Input {
  allowed_ranges: HashMap<String, Vec<Range>>,
  your_ticket: Ticket,
  nearby_tickets: Vec<Ticket>
}


fn read_input(file_name: &str) -> Input {
  let raw_input = std::fs::read_to_string(file_name).unwrap();

  let mut input = Input {
    allowed_ranges: HashMap::new(),
    your_ticket: Vec::new(),
    nearby_tickets: Vec::new()
  };

  let mut phase = 0;
  raw_input.lines().for_each(|l| {
    if l.is_empty() {
      phase += 1;
    } else if !COMMENT_REGEX.is_match(l) {
      match phase {
        0 => {
          let captures = ALLOWED_RANGES_REGEX.captures(l).unwrap();
          let name = captures.get(1).unwrap().as_str().to_string();
          let range1 = parse_range(captures.get(2).unwrap().as_str());
          let range2 = parse_range(captures.get(3).unwrap().as_str());
          input.allowed_ranges.insert(name, vec![range1, range2]);
        },
        1 => {
          input.your_ticket = parse_ticket(l);
        },
        2 => {
          input.nearby_tickets.push(parse_ticket(l));
        }
        _ => ()
     }
    }
  });

  input
}

fn parse_range(range: &str) -> Range {
  let mut split = range.split("-");
  Range(
    split.next().unwrap().parse::<i64>().unwrap(),
    split.next().unwrap().parse::<i64>().unwrap()
  )
}

fn parse_ticket(line: &str) -> Ticket {
  line.split(",").map(|v| v.parse::<i64>().unwrap()).collect::<Vec<_>>()
}
