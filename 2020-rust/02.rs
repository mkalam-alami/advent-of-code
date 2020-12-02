use std::fs;
use regex::Regex;

fn main() {
  let example = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
  println!("EXAMPLE");
  part1(&example);
  part2(&example);

  let input = fs::read_to_string("02.txt").unwrap();
  println!("INPUT");
  part1(&input);
  part2(&input);
}

fn part1(input: &str) {
  let mut valid_passwords = 0;

  for password_info in parse_password_infos(input) {
    let min = String::from(password_info[0]).parse::<i32>().unwrap();
    let max = String::from(password_info[1]).parse::<i32>().unwrap();
    let letter = String::from(password_info[2]).chars().next().unwrap();
    let password = String::from(password_info[4]);

    let mut found_letters = 0;
    for c in password.chars() {
      if c == letter {
        found_letters += 1;
      }
    }

    if found_letters >= min && found_letters <= max {
      // println!("VALID   {} {} {} {}", min, max, letter, password);
      valid_passwords += 1;
    } else {
      // println!("INVALID {} {} {} {}", min, max, letter, password);
    }
  }

  println!("-------------\nPT1. TOTAL VALID = {}\n", valid_passwords);
}

fn part2(input: &str) {
  let mut valid_passwords = 0;

  for password_info in parse_password_infos(input) {
    let index1 = String::from(password_info[0]).parse::<usize>().unwrap();
    let index2 = String::from(password_info[1]).parse::<usize>().unwrap();
    let letter = String::from(password_info[2]).chars().next().unwrap();
    let password = String::from(password_info[4]);

    let mut matches = 0;
    if password.chars().nth(index1 - 1).unwrap() == letter {
      matches += 1;
    }
    if password.chars().nth(index2 - 1).unwrap() == letter {
      matches += 1;
    }

    if matches == 1 {
      // println!("VALID   {} {} {} {}", index1, index2, letter, password);
      valid_passwords += 1;
    } else {
      // println!("INVALID {} {} {} {}", index1, index2, letter, password);
    }
  }

  println!("-------------\nPT2. TOTAL VALID = {}\n", valid_passwords);
}

fn parse_password_infos(input: &str) -> Vec<Vec<&str>> {
  let split_pattern = Regex::new(r"[ :-]").unwrap();
  let mut password_infos: Vec<Vec<&str>> = Vec::new();
  input.lines()
    .for_each(|line| password_infos.push(split_pattern.split(line).collect::<Vec<&str>>()));
  return password_infos;
}