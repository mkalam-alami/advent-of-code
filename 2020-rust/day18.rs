fn main() {
  let input = read_input("day18-example.txt");
  println!("EXAMPLE\n----------");
  part1(input);
}

fn part1(input: Vec<String>) {
  // TODO
}

fn parse_expression<'a>(input: &'a str) -> Expression<'a> {
  let left: ExpressionHand;
  let c = input.chars().nth(0).expect("fell off expression while parsing left");
  let mut next_char = 1;
  match c {
    '(' => {
      let closing_parenthesis = find_closing_parenthesis_index(&input);
      let child_expression = parse_expression(&input[0..closing_parenthesis]);
      left = ExpressionHand::EXPRESSION(&child_expression);
      next_char = closing_parenthesis + 1;
    },
    _ => left = ExpressionHand::VALUE(c.to_string().parse::<usize>().unwrap())
  }

  let operator = input.chars().nth(next_char).expect("fell off expression while parsing operator");

  let right: ExpressionHand;
  if next_char + 1 == input.len() - 1 {
    right = ExpressionHand::VALUE(input[next_char+1..input.len()].parse::<usize>().unwrap());
  } else {
    let child_expression = parse_expression(&input[next_char + 1..input.len()]);
    right = ExpressionHand::EXPRESSION(&child_expression);
  }

  Expression { left, right, operator } // FIXME I don't understand lifetimes :(
}

fn find_closing_parenthesis_index(input: &str) -> usize {
  let mut it = input.chars();
  let mut open_parenthesis_inside = 0;
  let mut index = 0;
  while let Some(c) = it.next() {
    index += 1;
    match c {
      '(' => open_parenthesis_inside += 1,
      ')' => {
        if open_parenthesis_inside > 0 {
          open_parenthesis_inside += 1;
        } else {
          break;
        }
      },
      _ => ()
    }
  }
  index
}

struct Expression<'a> {
  left: ExpressionHand<'a>,
  right: ExpressionHand<'a>,
  operator: char
}

enum ExpressionHand<'a> {
  VALUE(usize),
  EXPRESSION(&'a Expression<'a>)
}

fn read_input(file_name: &str) -> Vec<String> {
  std::fs::read_to_string(file_name).unwrap().lines().map(|l| l.replace(" ", "").to_string()).collect()
}
