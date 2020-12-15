use std::{fmt::Display, fs};
use regex::Regex;

fn main() {
  let input = read_input("14-example.txt");
  part1(&input);
}

fn part1(input: &Vec<Command>) {
  input.iter()
    .for_each(|c| println!("{}", c));
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