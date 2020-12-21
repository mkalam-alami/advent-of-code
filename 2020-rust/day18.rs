#[macro_use]
extern crate lazy_static;

mod day18parser;
use regex::Match;
use regex::Regex;
use bigdecimal::BigDecimal;

lazy_static! {
  static ref NUMBER_REGEX: Regex = Regex::new("^([0-9]+)$").unwrap();
  static ref NUMBER_IN_PARENTHESES_REGEX: Regex = Regex::new("\\(([0-9]+)\\)").unwrap();
  static ref SIMPLE_PLUS_EXPR_REGEX: Regex = Regex::new("([0-9]+)\\+([0-9]+)").unwrap();
  static ref SIMPLE_MULT_EXPR_REGEX: Regex = Regex::new("([0-9]+)\\*([0-9]+)").unwrap();
  static ref SUB_EXPRESSION_REGEX: Regex = Regex::new("\\(([0-9]+[+*][0-9+*]+)\\)").unwrap();
}

fn main() {
  let input = read_input("day18-examples.txt");
  println!("EXAMPLE\n----------");
  part1(&input, true);
  part2(&input, 1);

  let input = read_input("day18.txt");
  println!("\nINPUT\n----------");
  part1(&input, false);
  part2(&input, 0);
}

fn part1(input: &Vec<String>, verbose: bool) {
  let sum: usize = input.iter().map(|line| {
    let mut parser = day18parser::Parser::new(line.as_str());
    let result = parser.parse().evaluate();
    if verbose {
      println!("{} = {} = {}", line, parser.parse(), result);
    }
    result
  }).sum();
  println!("SUM: {}\n", sum);
}

fn part2(input: &Vec<String>, verbose_level: i8) {
  let sum: BigDecimal = input.iter().map(|line| {
    if verbose_level > 1 {
      println!("  {}", line);
    }
    let expression = evaluate_expression(line.clone(), verbose_level);
    if verbose_level > 0 {
      println!("{} = {}", line, expression);
    }
    expression
  }).sum();
  println!("SUM: {}", sum);
}

fn evaluate_expression(input: String, verbose_level: i8) -> BigDecimal {
  let mut expression = input.clone();

  while !NUMBER_REGEX.is_match(expression.as_str()) {
    expression = evaluate_step(expression);
    if verbose_level > 1 {
      println!("  {}", expression);
    }
  }

  BigDecimal::parse_bytes(expression.as_bytes(), 10).unwrap()
}

fn evaluate_step(expression: String) -> String {
  if let Some(captures) = NUMBER_IN_PARENTHESES_REGEX.captures(expression.as_str()) {
    return expression.replacen(strp(captures.get(0)), strp(captures.get(1)), 1);
  }

  if let Some(captures) = SUB_EXPRESSION_REGEX.captures(expression.as_str()) {
    let sub_expression = strp(captures.get(0));
    return expression.replacen(sub_expression,
    evaluate_expression(strp(captures.get(1)).to_string(), 0).to_string().as_str(), 1);
  }

  if let Some(captures) = SIMPLE_PLUS_EXPR_REGEX.captures(expression.as_str()) {
    // println!("added {}  +  {} to make {} {}", expression, captures.get(0).unwrap().a, usize(captures.get(1)), usize(captures.get(2)));
    return expression.replacen(strp(captures.get(0)),
      (decimal(captures.get(1)) + decimal(captures.get(2))).to_string().as_str(), 1);
    // println!("{}", expression);
  }

  if let Some(captures) = SIMPLE_MULT_EXPR_REGEX.captures(expression.as_str()) {
    // println!("{}", expression);
    return expression.replacen(strp(captures.get(0)),
      (decimal(captures.get(1)) * decimal(captures.get(2))).to_string().as_str(), 1);
  }

  panic!("no rule found to simplify: {}", expression);
}

fn strp(option: Option<Match>) -> &str {
  option.unwrap().as_str()
}

fn decimal(option: Option<Match>) -> BigDecimal {
  BigDecimal::parse_bytes(strp(option).as_bytes(), 10).unwrap()
}

fn read_input(file_name: &str) -> Vec<String> {
  std::fs::read_to_string(file_name).unwrap().lines().map(|l| l.replace(" ", "").to_string()).collect()
}
