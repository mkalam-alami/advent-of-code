#[macro_use]
extern crate lazy_static;

use std::{collections::HashMap, fmt::Display};
use regex::Regex;

lazy_static! {
  static ref COMMENT_REGEX: Regex = Regex::new("[a-z:]+$").unwrap();
  static ref ALLOWED_RANGES_REGEX: Regex = Regex::new("([^:]+): ([0-9-]+) or ([0-9-]+)").unwrap();
}

fn main() {
  let input = read_input("16-example.txt");
  println!("EXAMPLE\n----------");
  part1(&input);

  let input = read_input("16-example-2.txt");
  println!("\nEXAMPLE 2\n----------");
  part2(&input, "class");
  part2(&input, "row");
  part2(&input, "seat");

  let input = read_input("16-example-custom.txt");
  println!("\nEXAMPLE CUSTOM\n----------");
  part2(&input, "departure");

  let input = read_input("16.txt");
  println!("\nINPUT\n----------");
  part1(&input);
  part2(&input, "departure");
}

fn part1(input: &Input) {
  let error_rate: i64 = input.nearby_tickets.iter()
    .map(|t| t.error_rate(&input.allowed_ranges))
    .sum();

  println!("PT.1: {}", error_rate);
}

fn part2(input: &Input, keyword: &str) {
  let field_count = input.allowed_ranges.len();

  // Guess field names
  let mut guessing_table = GuessingTable::new(&input.allowed_ranges);
  input.nearby_tickets.iter()
    .enumerate()
    .filter(|(_, t)| t.is_valid(&input.allowed_ranges))
    .for_each(|(_t_index, t)| {
      for field_index in 0..field_count {
        // println!("{}", field_index);
        let incompatible_names = t.incompatible_names(field_index, &input.allowed_ranges);
        // if incompatible_names.len() > 0 {
        //   println!("ticket {} field {} value {} excludes '{}'", t_index, field_index, t.values.get(field_index).unwrap(), incompatible_names.iter().map(|n| n.to_string()).collect::<Vec<_>>().join(","));
        // }
        incompatible_names
          .iter()
          .for_each(|name| guessing_table.exclude(field_index, name));
      }
    });
  // println!("guessing results:\n{}", guessing_table);
  
  // Count matches from your ticket
  let result = input.your_ticket.values
    .iter()
    .enumerate()
    .filter(|(field_index, _)| guessing_table.get_name(field_index).starts_with(keyword))
    .map(|(_, value)| value)
    .product::<i64>();

  println!("PT.2: {}={}", keyword, result);
}

struct GuessingTable {
  data: HashMap<usize, Vec<String>>
}
impl Display for GuessingTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let mut result = write!(f, "");
      self.data.iter()
        .for_each(|(field_index, names)| {
          result = writeln!(f, "{}: {}", field_index, names.join(","));
        });
      result
    }
}
impl GuessingTable {
  fn new(allowed_ranges: &AllowedRanges) -> GuessingTable {
    let field_names = allowed_ranges.keys()
      .map(|s| s.to_owned())
      .collect::<Vec<String>>();
      
    let mut data = HashMap::new();
    field_names.iter()
      .enumerate()
      .for_each(|(index, _)| {
        data.insert(
          index,
          field_names.iter().map(|s| s.to_owned()).collect::<Vec<_>>());
      });
      
    GuessingTable { data }
  }
  fn exclude(&mut self, field_index: usize, name: &String) {
    let mut forced_solution: Option<String> = None;
    self.data.get_mut(&field_index).map(|names| {
      let was_multiple_solutions = names.len() > 1;
      if let Some(name_index) = names.iter().position(|p| p == name) {
        // println!("  excluding {} at index {}", name, field_index);
        names.remove(name_index);
      }
      if was_multiple_solutions && names.len() == 1 {
        forced_solution = Some(names.get(0).unwrap().clone());
      }
      if names.len() == 0 {
        panic!("nothing matches field index {}?!", field_index);
      }
    });

    if forced_solution.is_some() {
      let forced_solution_name = forced_solution.unwrap();
      // println!("   -> BINGO! {} is {}", field_index, forced_solution_name);
      for other_index in 0..(self.data.keys().len()) {
        if other_index != field_index {
          self.exclude(other_index, &forced_solution_name);
        }
      }
    }
  }
  fn get_name(&self, field_index: &usize) -> String {
    self.data.get(field_index).and_then(|names| {
      if names.len() == 1 {
        Some(names.get(0).unwrap().to_owned())
      } else {
        None
      }
    }).expect(
        &format!("{} names found for field index {}: {}",
          self.data.get(field_index).unwrap().len(),
          field_index,
          self.data.get(field_index).unwrap().join(",")))
  }
}

struct Range(i64, i64);

struct Ticket {
  values: Vec<i64>
}
impl Ticket {
  fn error_rate(&self, allowed_ranges: &AllowedRanges) -> i64 {
    self.values.iter()
      .filter(|value| {
        allowed_ranges.values().all(|ranges| !are_ranges_compatible(ranges, **value))
      })
      .sum()
  }
  fn is_valid(&self, allowed_ranges: &AllowedRanges) -> bool {
    self.values.iter()
      .all(|value| {
        allowed_ranges.values().any(|ranges| are_ranges_compatible(ranges, *value))
      })
  }
  fn incompatible_names<'a>(&self, field_index: usize, allowed_ranges: &'a AllowedRanges) -> Vec<&'a String> {
    let value = *self.values.get(field_index).unwrap();
    allowed_ranges
      .iter()
      .filter(|(_, ranges)| !are_ranges_compatible(ranges, value))
      .map(|(name, _)| {
        name
      })
      .collect()
  }
}

fn are_ranges_compatible(ranges: &Vec<Range>, value: i64) -> bool {
  ranges.iter().any(|range| value >= range.0 && value <= range.1)
}

type AllowedRanges = HashMap<String, Vec<Range>>;

struct Input {
  allowed_ranges: AllowedRanges,
  your_ticket: Ticket,
  nearby_tickets: Vec<Ticket>
}

fn read_input(file_name: &str) -> Input {
  let raw_input = std::fs::read_to_string(file_name).unwrap();

  let mut input = Input {
    allowed_ranges: AllowedRanges::new(),
    your_ticket: Ticket { values: Vec::new() },
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
  Ticket { values:  line.split(",").map(|v| v.parse::<i64>().unwrap()).collect::<Vec<_>>() }
}
