extern crate pancurses;

mod day17grid;

use std::fs;
use day17grid::{Coords4D, Grid4D};

static MAX_TURNS: usize = 6;

fn main() {
  let grid = read_input("17-example.txt", 3);
  println!("EXAMPLE\n------------");
  part1(&grid, 3); // 112
  part2(&grid, 3); // 848

  let grid = read_input("17.txt", 8);
  println!("\nINPUT\n------------");
  part1(&grid, 8); // 267
  part2(&grid, 8);
}

fn part1(grid: &Grid4D, initial_size: i64) {
  let mut current_grid = grid.clone();
  current_grid.use_fourth_dimension = false;

  for turn in 0..MAX_TURNS {
    let mut next_grid = current_grid.clone();

    for x in (-1 - turn as i64)..(initial_size + 1 + turn as i64) {
      for y in (-1 - turn as i64)..(initial_size + 1 + turn as i64) {
        for z in (-1 - turn as i64)..(initial_size + 1 + turn as i64) {
          let coords: Coords4D = (x,y,z,0);
          let active_neighbors = current_grid.count_active_neighbors(coords);
          let next_active = match current_grid.is_active(coords) {
            true => active_neighbors == 2 || active_neighbors == 3,
            false => active_neighbors == 3
          };
          // if next_active != current_grid.is_active(coords) {
          //   println!("({},{},{}) is now {}", coords.0, coords.1, coords.2, if next_active { "active" } else { "inactive" });
          // }
          next_grid.set_active(coords, next_active);
        }
      }
    }

    // println!("{}", next_grid.get_printable_slice(0, 0));

    current_grid = next_grid.clone();
  }

  println!("RESULT: {}", current_grid.count_active());
}

fn part2(grid: &Grid4D, initial_size: i64) {
  let mut current_grid = grid.clone();
  current_grid.use_fourth_dimension = true;

  for turn in 0..MAX_TURNS {
    let mut next_grid = current_grid.clone();

    for x in (-1 - turn as i64)..(initial_size + 1 + turn as i64) {
      for y in (-1 - turn as i64)..(initial_size + 1 + turn as i64) {
        for z in (-1 - turn as i64)..(initial_size + 1 + turn as i64) {
          for w in (-1 - turn as i64)..(initial_size + 1 + turn as i64) {
            let coords: Coords4D = (x,y,z,w);
            let active_neighbors = current_grid.count_active_neighbors(coords);
            let next_active = match current_grid.is_active(coords) {
              true => active_neighbors == 2 || active_neighbors == 3,
              false => active_neighbors == 3
            };
            next_grid.set_active(coords, next_active);
          }
        }
      }
    }

    // println!("{}", next_grid.get_printable_slice(0, 0));
    println!("Turn {} simulated", turn + 1);

    current_grid = next_grid.clone();
  }

  println!("RESULT: {}", current_grid.count_active());
}

fn read_input(file_name: &str, starting_size: usize) -> Grid4D {
  let mut grid = Grid4D::new(starting_size + MAX_TURNS);

  fs::read_to_string(file_name)
    .unwrap()
    .lines()
    .enumerate()
    .for_each(|(y, row)| {
      row.chars()
        .enumerate()
        .for_each(|(x, char)| grid.set_active((x as i64,y as i64,0,0), char == '#'));
    });

  grid
}
