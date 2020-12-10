use std::{collections::HashMap, fs};

fn main() {
  let adapters = read_input("10-example");
  println!("EXAMPLE\n-----------");
  part1(&adapters);
  part2(&adapters);

  let adapters = read_input("10-example-2");
  println!("\nEXAMPLE 2\n-----------");
  part1(&adapters);
  part2(&adapters);

  let adapters = read_input("10");
  println!("\nINPUT\n-----------");
  part1(&adapters);
  part2(&adapters);
}

fn part1(adapters: &Vec<i64>) {
  let differences = adapters.iter()
    .zip(adapters[1..].into_iter())
    .map(|(a1, a2)| a2 - a1)
    .fold((0, 1/*built-in adapter*/), |(n1, n3), n| {
      match n {
        1 => (n1 + 1, n3),
        3 => (n1, n3 + 1),
        _ => (n1, n3)
      }
    });
  println!("PT.1: {} x {} = {}", differences.0, differences.1, differences.0 * differences.1);
}

fn part2(adapters: &Vec<i64>) {
  let mut count_cache: HashMap<i64, i64> = HashMap::new();
  let count = count_arrangements(&0, &adapters[1..].to_vec(), &mut count_cache);
  println!("PT.2: {}", count);
}

fn count_arrangements(current_adapter: &i64, adapters: &Vec<i64>, count_cache: &mut HashMap<i64, i64>) -> i64 {
  if count_cache.contains_key(current_adapter) {
    return count_cache[current_adapter];
  }
  if adapters.len() <= 1 {
    return 1;
  }

  let acc = adapters.iter().take_while(|j| **j <= current_adapter + 3)
    .map(|compatible_adapter| {
      count_arrangements(
        compatible_adapter,
        &adapters.iter().filter_map(|j| if j > compatible_adapter { Some(*j) } else { None }).collect(),
        count_cache)
    })
    .sum();
  count_cache.insert(*current_adapter, acc);
  acc
}

fn read_input(file_name: &str) -> Vec<i64> {
  let start_joltage = vec![0 as i64];
  let mut adapters = fs::read_to_string(format!("{}{}", file_name, ".txt")).unwrap()
    .lines()
    .map(|line| line.parse::<i64>().unwrap())
    .chain(start_joltage)
    .collect::<Vec<i64>>();
  adapters.sort();
  return adapters;
}
