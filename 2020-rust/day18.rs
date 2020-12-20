mod day18parser;
use regex::Match;
use regex::Regex;

fn main() {
  let input = read_input("day18-examples.txt");
  println!("EXAMPLE\n----------");
  part1(&input, true);
  part2(&input, true);

  // let input = read_input("day18.txt");
  // println!("\nINPUT\n----------");
  // part1(input, false);
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
  println!("SUM: {}", sum);
}

fn part2(input: &Vec<String>, verbose: bool) {
  let number = Regex::new("^([0-9]+)$").unwrap();
  let number_in_parentheses = Regex::new("\\(([0-9]+)\\)").unwrap();
  let simple_plus_expr = Regex::new("([0-9]+)\\+([0-9]+)").unwrap();
  let simple_mult_expr = Regex::new("([0-9]+)\\*([0-9]+)").unwrap();

  let sum: usize = input.iter().map(|line| {
    let mut expression = line.clone();

    while !number.is_match(expression.as_str()) {
      println!("{}", expression);

      while let Some(captures) = simple_plus_expr.captures(expression.as_str()) {
        // println!("added {}  +  {} to make {} {}", expression, captures.get(0).unwrap().a, usize(captures.get(1)), usize(captures.get(2)));
        expression = expression.replace(strp(captures.get(0)),
          (usize(captures.get(1)) + usize(captures.get(2))).to_string().as_str() );
        if let Some(captures) = number_in_parentheses.captures(expression.as_str()) {
          expression = expression.replace(strp(captures.get(0)), strp(captures.get(1)));
        }
        println!("{}", expression);
      }

      while let Some(captures) = simple_mult_expr.captures(expression.as_str()) {
        expression = expression.replace(strp(captures.get(0)),
          (usize(captures.get(1)) * usize(captures.get(2))).to_string().as_str() );
          println!("{}", expression);
        if let Some(captures) = number_in_parentheses.captures(expression.as_str()) {
          expression = expression.replace(strp(captures.get(0)), strp(captures.get(1)));
        }
        println!("{}", expression);
      }
    }

    if verbose {
      println!("{} = {}", line, expression);
    }

    expression.parse::<usize>().unwrap()
  }).sum();
  println!("SUM: {}", sum);
}

fn strp(option: Option<Match>) -> &str {
  option.unwrap().as_str()
}

fn usize(option: Option<Match>) -> usize {
  strp(option).parse::<usize>().unwrap()
}

fn read_input(file_name: &str) -> Vec<String> {
  std::fs::read_to_string(file_name).unwrap().lines().map(|l| l.replace(" ", "").to_string()).collect()
}
