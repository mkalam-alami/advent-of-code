#[macro_use]
extern crate lazy_static;

mod day04rules;

use std::fs;
use std::time;
use std::collections::HashMap;
use regex;
use day04rules;

type Passport = HashMap<String, String>;

fn main() {
  let start = time::Instant::now();

  unit_tests();

  let passports = load_passports("04-example.txt");
  println!("EXAMPLE\n------------");
  part1(&passports);
  part2(&passports);

  let passports = load_passports("04.txt");
  println!("INPUT\n------------");
  part1(&passports);
  part2(&passports);

  println!("Completed in {}ms", start.elapsed().as_millis());
}

fn unit_tests() {
  println!("UNIT TESTS\n------------");
  println!("byr: 1950 is {}", rules04::is_byr_valid("1950"));
  println!("byr: 1900 is {}", rules04::is_byr_valid("1900"));
  println!("hgt: 160cm is {}", rules04::is_hgt_valid("160cm"));
  println!("hgt: 300cm is {}", rules04::is_hgt_valid("300cm"));
  println!("hgt: 65in is {}", rules04::is_hgt_valid("65in"));
  println!("hgt: 300in is {}", rules04::is_hgt_valid("300in"));
  println!("hgt: 300 is {}", rules04::is_hgt_valid("300"));
  println!("hcl: #123456 is {}", rules04::is_hcl_valid("#123456"));
  println!("hcl: #1234567 is {}", rules04::is_hcl_valid("#1234567"));
  println!("hcl: 123456 is {}", rules04::is_hcl_valid("123456"));
  println!("ecl: amb is {}", rules04::is_ecl_valid("amb"));
  println!("ecl: ambu is {}", rules04::is_ecl_valid("ambu"));
  println!("pid: 123456789 is {}", rules04::is_pid_valid("123456789"));
  println!("pid: 12345678 is {}", rules04::is_pid_valid("12345678"));
  println!("");
}

fn part1(passports: &Vec<Passport>) {
  let valid_passport_count = passports.iter()
    .map(|passport| is_passport_valid_pt1(passport) as u64)
    .sum::<u64>();

  println!("PT. 1 RESULT: {}", valid_passport_count);
  println!();
}

fn part2(passports: &Vec<Passport>) {
  let valid_passport_count = passports.iter()
    .map(|passport| (
      is_passport_valid_pt1(passport) &&
      rules04::is_byr_valid(passport.get("byr").unwrap_or(&"".to_string()).as_str()) &&
      rules04::is_iyr_valid(passport.get("iyr").unwrap_or(&"".to_string()).as_str()) &&
      rules04::is_eyr_valid(passport.get("eyr").unwrap_or(&"".to_string()).as_str()) &&
      rules04::is_hgt_valid(passport.get("hgt").unwrap_or(&"".to_string()).as_str()) &&
      rules04::is_hcl_valid(passport.get("hcl").unwrap_or(&"".to_string()).as_str()) &&
      rules04::is_ecl_valid(passport.get("ecl").unwrap_or(&"".to_string()).as_str()) &&
      rules04::is_pid_valid(passport.get("pid").unwrap_or(&"".to_string()).as_str())
     ) as u64)
    .sum::<u64>();

  println!("PT. 2 RESULT: {}", valid_passport_count);
  println!();
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
