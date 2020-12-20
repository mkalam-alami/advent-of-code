#[macro_use]
extern crate lazy_static;

use std::{collections::HashMap, println};
use std::fs;
use regex::Regex;

lazy_static! {
  static ref RULE_ELEMENT_SEPARATOR: Regex = regex::Regex::new("( contain |, )").unwrap();
  static ref PLURAL: Regex = Regex::new("s$").unwrap();
}

fn main() {
  println!("EXAMPLE\n------------");
  let input = load_input("07-example.txt");
  part1(&input);
  part2(&input);

  println!("\nEXAMPLE 2\n------------");
  let input = load_input("07-example-2.txt");
  part1(&input);
  part2(&input);

  println!("\nINPUT\n------------");
  let input = load_input("07.txt");
  part1(&input);
  part2(&input);
}

fn part1(all_colors: &HashMap<String, BagRule>) {
  // all_colors.values().for_each(|color| println!("{}", color.color));
  // println!("--------");
  let result = all_colors.values()
    .map(|bag_color| bag_can_contain(&bag_color, "shiny gold bag", &all_colors) as i32)
    .sum::<i32>();
  println!("PT.1: {}", result);
}

fn part2(all_colors: &HashMap<String, BagRule>) {
  let count = count_contents("shiny gold bag", all_colors);
  println!("PT.2: {}", count);
}

fn count_contents(color_name: &str, all_colors: &HashMap<String, BagRule>) -> i32 {
  return all_colors.get(color_name).expect(color_name)
    .required_contents
    .iter()
    .map(|(color_name, amount)| amount * (count_contents(color_name, all_colors) + 1))
    .sum::<i32>();
}

struct BagRule {
  color_name: String,
  required_contents: HashMap<String, i32>
}

fn bag_can_contain(bag_rule: &BagRule, color_name: &str, all_colors: &HashMap<String, BagRule>) -> bool {
  if bag_rule.required_contents.contains_key(color_name) {
    // println!("{} can contain {}", bag_color.color, color_name);
    return true;
  } else {
    return bag_rule.required_contents.iter()
      .any(|allowed_color| {
        if !all_colors.contains_key(allowed_color.0) {
          panic!("{}", allowed_color.0);
        }
        bag_can_contain(&all_colors[allowed_color.0], color_name, all_colors)
      });
  }
}

fn load_input(file_name: &str) -> HashMap<String, BagRule> {
  let input = fs::read_to_string(file_name).unwrap();
  return input.lines()
    .map(|line| parse_rule(line))
    .map(|bag_color| (bag_color.color_name.clone(), bag_color))
    .collect::<HashMap<String, BagRule>>();
}

fn parse_rule<'a>(line: &str) -> BagRule {
  let rule_elements = RULE_ELEMENT_SEPARATOR
      .split(line.replace(".", "").as_str())
      .filter(|element| *element != "no other bags")
      .map(|rule_element| PLURAL.replace(rule_element, "").to_string())
      .collect::<Vec<String>>();

  let color_name = rule_elements[0].to_string();
  let required_contents = rule_elements[1..].iter()
    .map(|element| element.split_at(2))
    .map(|(amount, color)| (color.to_string(), amount.trim().parse::<i32>().unwrap()))
    .collect::<HashMap<String, i32>>();

  return BagRule {
    color_name,
    required_contents
  };
}
