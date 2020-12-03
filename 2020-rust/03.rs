use std::fs;
use std::ops;

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

struct Vec2(usize, usize); // Using "tuple struct" syntax https://doc.rust-lang.org/1.9.0/book/structs.html#tuple-structs
impl ops::Add<&Vec2> for Vec2 {
  type Output = Vec2;

  fn add(self, point: &Vec2) -> Vec2 {
    return Vec2(self.0 + point.0, self.1 + point.1);
  }
}

struct Forest {
  map: Vec<String>,
  width: usize,
  height: usize
}
trait TileMap {
  fn get_tile(&self, coords: &Vec2) -> char;
}
impl TileMap for Forest {
  fn get_tile(&self, coords: &Vec2) -> char {
    return self.map[coords.1].chars().nth(coords.0).unwrap();
  }
}

fn part1(forest: &Forest) {
  let result = trees_encountered(forest, &Vec2(3, 1));
  println!("PT. 1 RESULT: {}", result);
  println!();
}

fn part2(forest: &Forest) {
  let slopes = [Vec2(1, 1), Vec2(3, 1), Vec2(5, 1), Vec2(7, 1), Vec2(1, 2)];

  let result = slopes.iter()
    .map(|slope| trees_encountered(forest, slope))
    .fold(1, |acc, trees| acc * trees);

  println!("PT2. RESULT: {}", result);
  println!();
}

fn trees_encountered(forest: &Forest, slope: &Vec2) -> usize {
  let mut world_pos = Vec2(0, 0);

  let mut trees_encountered = 0;
  while world_pos.1 < forest.height {
    trees_encountered += is_tree(&forest, &world_pos) as usize;
    world_pos = world_pos + slope;
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

fn is_tree(forest: &Forest, world_pos: &Vec2) -> bool {
  let forest_coords = Vec2(world_pos.0 % forest.width, world_pos.1 % forest.height);
  return forest.get_tile(&forest_coords) == '#';
}
