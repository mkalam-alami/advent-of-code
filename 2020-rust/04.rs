use std::fs;
use std::collections::HashMap;
use regex;

type Passport = HashMap<String, String>;

fn main() {
  let passports = load_passports("04-example.txt");
  println!("EXAMPLE\n------------");
  part1(&passports);
  part2(&passports);

  let passports = load_passports("04.txt");
  println!("INPUT\n------------");
  part1(&passports);
  part2(&passports);
}

fn part1(passports: &Vec<Passport>) {
  let valid_passport_count = passports.iter()
    .map(|passport| is_passport_valid_pt1(passport) as u64)
    .sum::<u64>();

  println!("PT. 1 RESULT: {}", valid_passport_count);
  println!();
}

fn part2(_passports: &Vec<Passport>) {
  // let valid_passport_count = passports.iter()
  //   .map(|passport| is_passport_valid_pt2(passport) as u64)
  //   .sum::<u64>();

  // println!("PT. 2 RESULT: {}", valid_passport_count);
  // println!();
}

fn is_passport_valid_pt1(passport: &Passport) -> bool {
  let is_valid = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
    .iter()
    .all(|key| passport.contains_key(&key.to_string()));
  // println!("{} is {}valid", passport.get("pid").unwrap_or(&"unknown".to_string()), if is_valid { "" } else { "not " });
  return is_valid;
}

fn load_passports(file_name: &str) -> Vec<Passport> {
  let passport_separator = regex::Regex::new("\r\n\r\n").unwrap();

  let input = fs::read_to_string(file_name).unwrap();
  return passport_separator.split(&input)
    .map(|passport_line| parse_passport_line(passport_line))
    .collect();
}

fn parse_passport_line(passport_line: &str) -> Passport {
  let info_separator = regex::Regex::new("[ \r\n]+").unwrap();
  let key_value_separator = regex::Regex::new(":").unwrap();

  let mut passport = Passport::new();
  info_separator.split(passport_line)
    .map(|token| key_value_separator.split(token).collect::<Vec<&str>>())
    .for_each(|entry| { passport.insert(entry[0].to_string(), entry[1].to_string()); });
  return passport;
}
