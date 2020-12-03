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

struct Forest {
  map: Vec<String>,
  width: usize,
  height: usize
}

trait TileMap {
  fn get_tile(&self, x: usize, y: usize) -> char;
}

impl TileMap for Forest {
  fn get_tile(&self, x: usize, y: usize) -> char {
    return self.map[y].chars().nth(x).unwrap();
  }
}

fn part1(forest: &Forest) {
  let result = trees_encountered(forest, &(3, 1));
  println!("PT. 1 RESULT: {}", result);
  println!();
}

fn part2(forest: &Forest) {
  let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

  let result = slopes.iter()
    .map(|slope| trees_encountered(forest, slope))
    .fold(1, |acc, trees| acc * trees);

  println!("PT2. RESULT: {}", result);
  println!();
}

fn trees_encountered(forest: &Forest, slope: &(usize, usize)) -> usize {
  let mut world_pos: (usize, usize) = (0, 0);

  let mut trees_encountered = 0;
  while world_pos.1 < forest.height {
    trees_encountered += is_tree(&forest, &world_pos) as usize;
    world_pos.0 += slope.0;
    world_pos.1 += slope.1;
  }

  println!("Trees encountered (slope of {}x{}): {}", slope.0, slope.1, trees_encountered);
  return trees_encountered;
}

fn load_forest(file_name: &str) -> Forest {
  let input = fs::read_to_string(file_name).unwrap();
  let map = input.lines().map(|s| s.to_string()).collect::<Vec<String>>();
  let width = map[0].len();
  let height = map.len();
  return Forest { map, width, height };
}

fn is_tree(forest: &Forest, world_pos: &(usize, usize)) -> bool {
  let forest_x = world_pos.0 % forest.width;
  let forest_y = world_pos.1 % forest.height;

  return forest.get_tile(forest_x, forest_y) == '#';
}
