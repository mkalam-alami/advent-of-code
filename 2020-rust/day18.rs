mod day18parser;

fn main() {
  let input = read_input("day18-examples.txt");
  println!("EXAMPLE\n----------");
  part1(input, true);

  let input = read_input("day18.txt");
  println!("\nINPUT\n----------");
  part1(input, false);
}

fn part1(input: Vec<String>, verbose: bool) {
  let sum: usize = input.iter().map(|line| {
    let mut parser = day18parser::Parser::new(line.as_str());
    let result = parser.parse().evaluate();
    if verbose {
      println!("{} = {}", line, result);
    }
    result
  }).sum();
  println!("SUM: {}", sum);
}

fn read_input(file_name: &str) -> Vec<String> {
  std::fs::read_to_string(file_name).unwrap().lines().map(|l| l.replace(" ", "").to_string()).collect()
}
