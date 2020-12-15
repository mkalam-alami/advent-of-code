#[macro_use]
extern crate lazy_static;

use std::{collections::HashMap, fmt::Display, fs};
use regex::Regex;

lazy_static! {
  static ref X: Regex = regex::Regex::new("X").unwrap();
}

fn main() {
  let input = read_input("14-example.txt");
  run(&input, &apply_mask_pt1);

  let input = read_input("14-example-2.txt");
  run(&input, &apply_mask_pt2);

  // let input = read_input("14.txt");
  // run(&input, &apply_mask_pt1);
}

fn run(input: &Vec<Command>, memory_writer: &dyn Fn(&mut HashMap<i64, i64>, &String, &Write) -> ()) {
  let mut current_mask = String::new();
  let mut memory: HashMap<i64, i64> = HashMap::new();
  input.iter()
    .for_each(|command| {
      // println!("{}", command);
      if command.mask.is_some() {
        current_mask = command.mask.as_ref().unwrap().clone();
      } else {
        let write = command.write.as_ref().unwrap();
        memory_writer(&mut memory, &current_mask, &write);
      }
    });

  println!("RESULT: {}", memory.values().sum::<i64>());
}

fn apply_mask_pt1(memory: &mut HashMap<i64, i64>, mask: &String, write: &Write) -> () {
  let mut mask_chars = mask.chars();
  let input = to_binary(write.value);
  let mut input_chars = input.chars();

  // println!("{} {}", mask, input);
  let mut output = String::new();
  loop {
    let mask_char = mask_chars.next();
    let input_char = input_chars.next();
    if mask_char.is_none() {
      break;
    }

    output.push(match mask_char.unwrap() {
      'X' => input_char.unwrap(),
      _ =>mask_char.unwrap()
    });
  }

  // println!(" ==> {}", output);
  memory.insert(write.address, from_binary(&output));
}

fn apply_mask_pt2(memory: &mut HashMap<i64, i64>, mask: &String, write: &Write) -> () {
  let mut mask_chars = mask.chars();
  let address = to_binary(write.address);
  let mut address_chars = address.chars();

  // println!("{} {}", mask, input);
  let mut masked_address = String::new();
  loop {
    let mask_char = mask_chars.next();
    let address_char = address_chars.next();
    if mask_char.is_none() {
      break;
    }

    masked_address.push(match mask_char.unwrap() {
      '0' => address_char.unwrap(),
      _ => mask_char.unwrap()
    });
  }

  // println!(" ==> {}", output);
  expand_floating_bits(&masked_address)
    .iter()
    .for_each(|address| { memory.insert(from_binary(address), write.value); });
}

fn expand_floating_bits(input: &String) -> Vec<String> {
  let _x_count = X.captures(input).unwrap().len();
  let result: Vec<String> = Vec::new();

  // TODO

  result
}

fn to_binary(input: i64) -> String {
  format!("{:#038b} ", input).split_at(2).1.to_string()
}

fn from_binary(input: &String) -> i64 {
  i64::from_str_radix(input.as_str(), 2).unwrap()
}

struct Command {
  mask: Option<String>,
  write: Option<Write>
}
impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      let default_mask_contents = "".to_string();
      let mask_contents = self.mask.as_ref().unwrap_or(&default_mask_contents);

      let default_write_contents = Write { address: -1, value: -1 };
      let write_contents = self.write.as_ref().unwrap_or(&default_write_contents);

      write!(f, "Command[mask={},write={}]", mask_contents, write_contents)
    }
}
struct Write {
  address: i64,
  value: i64
}
impl Display for Write {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      if self.address > 0 {
        write!(f, "Write[address={},value={}]", self.address, self.value)
      } else {
        write!(f, "")
      }
    }
}

fn read_input(file_name: &str) -> Vec<Command> {
  let write_regex = Regex::new("mem\\[([0-9]+)\\] = ([0-9]+)").unwrap();

  fs::read_to_string(file_name).unwrap()
    .lines()
    .map(|l| {
      if l.starts_with("mask") {
        Command {
          mask: Option::Some(l.split_at(7).1.to_string()),
          write: Option::None
        }
      } else {
        let matches = write_regex.captures(l).unwrap();
        Command {
          mask: Option::None,
          write: Option::Some(Write {
            address: matches.get(1).unwrap().as_str().parse().unwrap(),
            value: matches.get(2).unwrap().as_str().parse().unwrap(),
          })
        }
      }
    })
    .collect()
}