use std::fs;

fn main() {
  let forest_example = load_forest("03-example.txt");
  println!("EXAMPLE\n------------");
  part1(&forest_example);
  part2(&forest_example);

  let forest = load_forest("03.txt");
  println!("INPUT\n------------");
  part1(&forest);
  part2(&forest);
}

fn part1(forest: &Vec<String>) {
  let result = trees_encountered(forest, 3, 1);
  println!("PT. 1 RESULT: {}", result);
  println!();
}

fn part2(forest: &Vec<String>) {
  let mut result = trees_encountered(forest, 1, 1);
  result *= trees_encountered(forest, 3, 1);
  result *= trees_encountered(forest, 5, 1);
  result *= trees_encountered(forest, 7, 1);
  result *= trees_encountered(forest, 1, 2);
  println!("PT2. RESULT: {}", result);
  println!();
}

fn trees_encountered(forest: &Vec<String>, slope_x: usize, slope_y: usize) -> usize {
  let mut world_x = 0;
  let mut world_y = 0;
  let forest_height = forest.len();

  let mut trees_encountered = 0;
  while world_y < forest_height {
    let tree_encountered = is_tree(&forest, &world_x, &world_y);
    if tree_encountered {
      trees_encountered += 1;
      // println!("Tree found at {} {}", world_x, world_y);
    }
    world_x += slope_x;
    world_y += slope_y;
  }

  println!("Trees encountered (slope of {}x{}): {}", slope_x, slope_y, trees_encountered);
  return trees_encountered;
}

fn load_forest(file_name: &str) -> Vec<String> {
  let input = fs::read_to_string(file_name).unwrap();
  return input.lines().map(|s| s.to_string()).collect();
}


fn is_tree(forest: &Vec<String>, world_x: &usize, world_y: &usize) -> bool {
  let forest_width = forest[0].len();
  let forest_height = forest.len();

  let forest_x = world_x % forest_width;
  let forest_y = world_y % forest_height;

  return forest[forest_y].chars().nth(forest_x).unwrap() == '#';
}