#[macro_use]
extern crate lazy_static;

mod day18parser;
use regex::Match;
use regex::Regex;
use bigdecimal::BigDecimal;

lazy_static! {
  static ref NUMBER: Regex = Regex::new("^([0-9]+)$").unwrap();
  static ref NUMBER_IN_PARENTHESES: Regex = Regex::new("\\(([0-9]+)\\)").unwrap();
  static ref SIMPLE_PLUS_EXPR: Regex = Regex::new("([0-9]+)\\+([0-9]+)").unwrap();
  static ref SIMPLE_MULT_EXPR: Regex = Regex::new("([0-9]+)\\*([0-9]+)").unwrap();
  static ref COMPLEX_EXPR_IN_PARENTHESES: Regex = Regex::new("\\([0-9+*]{4,}\\)").unwrap();
}

fn main() {
  let input = read_input("day18-examples.txt");
  println!("EXAMPLE\n----------");
  part1(&input, true);
  part2(&input, true);

  let input = read_input("day18.txt");
  println!("\nINPUT\n----------");
  part1(&input, false);
  part2(&input, false); // more than 145129606953052
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

fn part2(input: &Vec<String>, verbose: bool) {
  let sum: BigDecimal = input.iter().map(|line| {
    evaluate_expression(line.clone(), verbose)
  }).sum();
  println!("SUM: {}", sum);
}

fn evaluate_expression(input: String, verbose: bool) -> BigDecimal {
  let mut expression = input.clone();

  while !NUMBER.is_match(expression.as_str()) {
    // println!("{}", expression);

    while let Some(captures) = COMPLEX_EXPR_IN_PARENTHESES.captures(expression.as_str()) {
      let inside_expression = strp(captures.get(0));
      expression = expression.replace(inside_expression,
      evaluate_expression(inside_expression[1..inside_expression.len()-1].to_string(), false).to_string().as_str());
    }

    while let Some(captures) = SIMPLE_PLUS_EXPR.captures(expression.as_str()) {
      // println!("added {}  +  {} to make {} {}", expression, captures.get(0).unwrap().a, usize(captures.get(1)), usize(captures.get(2)));
      expression = expression.replace(strp(captures.get(0)),
        (decimal(captures.get(1)) + decimal(captures.get(2))).to_string().as_str() );
      if let Some(captures) = NUMBER_IN_PARENTHESES.captures(expression.as_str()) {
        expression = expression.replace(strp(captures.get(0)), strp(captures.get(1)));
      }
      // println!("{}", expression);
    }

    while let Some(captures) = SIMPLE_MULT_EXPR.captures(expression.as_str()) {
      expression = expression.replace(strp(captures.get(0)),
        (decimal(captures.get(1)) * decimal(captures.get(2))).to_string().as_str() );
        // println!("{}", expression);
      if let Some(captures) = NUMBER_IN_PARENTHESES.captures(expression.as_str()) {
        expression = expression.replace(strp(captures.get(0)), strp(captures.get(1)));
      }
      // println!("{}", expression);
    }
  }

  if verbose {
    println!("{} = {}", input, expression);
  }

  BigDecimal::parse_bytes(expression.as_bytes(), 10).unwrap()
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
