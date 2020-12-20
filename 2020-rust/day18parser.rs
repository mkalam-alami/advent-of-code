use std::fmt::Display;

pub struct Expression {
  left: ExpressionHand,
  right: ExpressionHand,
  operator: char
}

impl Expression {
  pub fn evaluate(&self) -> usize {
    match self.operator {
      '*' => self.left.evaluate() * self.right.evaluate(),
      '+' => self.left.evaluate() + self.right.evaluate(),
      _ => panic!("unknown operator {}", self.operator)
    }
  }
}
impl Display for Expression {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    write!(f, "E({} {} {})", self.left, self.operator, self.right)
  }
}

pub enum ExpressionHand {
  VALUE(usize),
  EXPRESSION(Box<Expression>)
}

impl ExpressionHand {
  fn evaluate(&self) -> usize {
    match self {
      ExpressionHand::VALUE(v) => *v,
      ExpressionHand::EXPRESSION(expression) => expression.evaluate()
    }
  }
}
impl Display for ExpressionHand {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    match self {
      ExpressionHand::VALUE(v) => write!(f, "{}", v),
      ExpressionHand::EXPRESSION(expression) => write!(f, "{}", expression)
    }
  }
}

pub struct Parser<'a> {
  pub input: &'a str
}

impl <'a> Parser<'a> {

  pub fn new(input: &'a str) -> Parser<'a> {
    Parser { input }
  }

  pub fn parse(&mut self) -> Expression {
    self.parse_expression(self.input.chars()
      .rev()
      .map(|l| match l { '(' => ')', ')' => '(', _ => l })
      .collect::<String>()
      .as_str())
  }

  fn parse_expression(&self, input: &'a str) -> Expression {
    // println!("parsing: {}", input);
    let left: ExpressionHand;
    let c = input.as_bytes()[0] as char;
    let mut next_char = 1;
    match c {
      '(' => {
        let closing_parenthesis = self.find_closing_parenthesis_index(input);
        // println!("inside parentheses: {} ({}=> {})", &input[1..closing_parenthesis - 1], input, closing_parenthesis);
        let child_expression: Expression = self.parse_expression(&input[1..closing_parenthesis - 1]);
        left = ExpressionHand::EXPRESSION(Box::from(child_expression));
        next_char = closing_parenthesis;
      },
      _ => {
        // println!("left is {}", c);
        left = ExpressionHand::VALUE(c.to_string().parse::<usize>().unwrap());
      }
    }

    if next_char >= input.len() {
      return Expression { left, right: ExpressionHand::VALUE(0), operator: '+' };
    }

    // println!("searching operator at {} of input {}", next_char, input);
    let operator = input.chars().nth(next_char).expect("fell off expression while parsing operator");

    let right: ExpressionHand;
    let remaining = &input[next_char + 1..input.len()];
    if next_char + 1 == input.len() - 1 {
      // println!("right is {}", remaining);
      right = ExpressionHand::VALUE(remaining.parse::<usize>().unwrap());
    } else {
      // println!("right is sub-expression {}", remaining);
      let child_expression: Expression = self.parse_expression(remaining);
      right = ExpressionHand::EXPRESSION(Box::from(child_expression));
    }

    Expression { left, right, operator }
  }

  fn find_closing_parenthesis_index(&self, input: &str) -> usize {
    let mut it = input.chars();
    let mut open_parenthesis_inside = 0;
    let mut index = 0;
    while let Some(c) = it.next() {
      index += 1;
      // println!("{}", c);
      match c {
        '(' => open_parenthesis_inside += 1,
        ')' => {
          if open_parenthesis_inside > 1 {
            open_parenthesis_inside -= 1;
          } else {
            break;
          }
        },
        _ => ()
      }
    }
    index
  }

}
