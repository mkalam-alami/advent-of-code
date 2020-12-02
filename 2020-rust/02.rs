/**
 * Alternate implementations:
 * - Using structures & traits: https://gist.github.com/felippemr/6fc930a438613480b856b366b9e25e0b
 * - Using filters & multi-valued return/destructuring: https://github.com/cXVpbnQ/aoc-rust/blob/main/day02/src/main.rs
 */

use std::fs;
use std::time;
use regex::Regex;

fn main() {
  let start = time::Instant::now();

  let example = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";
  println!("EXAMPLE");
  part1(&example);
  part2(&example);

  let input = fs::read_to_string("02.txt").unwrap();
  println!("INPUT");
  part1(&input);
  part2(&input);

  println!("Completed in {}ms", start.elapsed().as_millis());
}

fn part1(input: &str) {
  let mut valid_passwords = 0;

  for password_info in parse_password_infos(input) {
    let min = password_info[0].parse::<i32>().unwrap();
    let max = password_info[1].parse::<i32>().unwrap();
    let letter = password_info[2].chars().next().unwrap();
    let password = password_info[4].chars();

    let mut found_letters = 0;
    // Alternative: Regex::from_str(letter).unwrap().find_iter(password).for_each(|_| found_letters += 1);
    for c in password {
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
    let index1 = password_info[0].parse::<usize>().unwrap();
    let index2 = password_info[1].parse::<usize>().unwrap();
    let letter = password_info[2].chars().next().unwrap();
    let password = password_info[4];

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