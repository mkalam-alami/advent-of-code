use std::{collections::HashMap, time::Instant};

fn main() {
  let start = Instant::now();

  let input = vec![0,3,6];
  println!("EXAMPLE\n----------");
  run(&input, 2020);
  run(&input, 30000000);

  let input = vec![2,1,10,11,0,6];
  println!("\nINPUT\n----------");
  run(&input, 2020);
  run(&input, 30000000);

  println!("\nCompleted in {}ms", start.elapsed().as_millis());
}

fn run(input: &Vec<usize>, n: usize) {
  let mut history: HashMap<usize, usize> = HashMap::new();
  input.iter()
    .enumerate()
    .for_each(|(turn, n)| {
      if turn + 1 != input.len() {
        history.insert(*n, turn + 1);
      }
    });

  let mut last_said = *input.iter().last().unwrap();
  for turn in (input.len() + 1)..(n + 1) {
    let n;
    if history.contains_key(&last_said) {
      n = turn - history[&last_said] - 1;
    } else {
      n = 0;
    }

    history.insert(last_said, turn - 1);
    last_said = n;
  }

  println!("RESULT: {}", last_said);
}
