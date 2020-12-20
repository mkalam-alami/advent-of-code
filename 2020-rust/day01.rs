use std::fs;

fn main() {
  let input = fs::read_to_string("01.txt")
    .expect("failed to read input");
  let numbers: Vec<_> = input.lines()
    .map(|line| line.parse::<i32>().expect("faild to parse number"))
    .collect();
  let number_count = numbers.len();

  for n1 in 0..number_count {
    for n2 in 0..number_count {
      if n1 != n2 && numbers[n1] + numbers[n2] == 2020 {
        println!("Match found: {} x {} = {}", numbers[n1], numbers[n2], numbers[n1] * numbers[n2]);
      }
      for n3 in 0..number_count {
        if n1 != n3 && n2 != n3 && numbers[n1] + numbers[n2] + numbers[n3] == 2020 {
          println!("Match found: {} x {} x {} = {}", numbers[n1], numbers[n2], numbers[n3], numbers[n1] * numbers[n2] * numbers[n3]);
        }
      }
    }
  }

}