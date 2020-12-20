use std::collections::HashMap;

pub type Coords4D = (i64, i64, i64, i64);

pub struct Grid4D {
  pub size: i64,
  internal_size: i64,
  cubes: HashMap<i64, bool>,
  pub use_fourth_dimension: bool
}

impl Grid4D {
  pub fn new(size: usize) -> Grid4D {
    let internal_size = (size + 1) * 2;
    let mut cubes = HashMap::new();
    cubes.reserve(internal_size * internal_size * internal_size);
    Grid4D { size: size as i64, internal_size: internal_size as i64, cubes, use_fourth_dimension: false }
  }

  pub fn clone(&self) -> Grid4D {
    Grid4D { size: self.size, internal_size: self.internal_size, cubes: self.cubes.clone(), use_fourth_dimension: self.use_fourth_dimension }
  }

  pub fn is_active(&self, coords: Coords4D) -> bool {
    *self.cubes.get(&self.coords_to_index(coords)).unwrap_or(&false)
  }

  pub fn set_active(&mut self, coords: Coords4D, active: bool) {
    // println!("set {}active: ({},{},{})", if active { "" } else { "in" }, coords.0, coords.1, coords.2);
    self.cubes.insert(self.coords_to_index(coords), active);
  }

  pub fn _get_printable_slice(&self, z: i64, w: i64) -> String {
    let mut slice = String::new();
    slice.reserve((self.size * (self.size + 1) * 2) as usize);
    for y in -self.size..self.size {
      for x in -self.size..self.size {
        slice.push(if self.is_active((x,y,z,w)) { '#' } else { '.' });
      }
      slice.push('\n');
    }
    slice
  }

  pub fn count_active_neighbors(&self, (x,y,z,w): Coords4D) -> i64 {
    let mut count = 0;
    for xn in x-1..x+2 {
      for yn in y-1..y+2 {
        for zn in z-1..z+2 {
          if self.use_fourth_dimension {
            for wn in w-1..w+2 {
              if !(xn == x && yn == y && zn == z && wn == w) {
                count += self.is_active((xn, yn, zn, wn)) as i64;
              }
            }
          } else {
            if !(xn == x && yn == y && zn == z) {
              count += self.is_active((xn, yn, zn, 0)) as i64;
            }
          }
        }
      }
    }
    // if count > 0 {
    //   println!("{} are active around {},{},{}", count, x, y, z);
    // }
    count
  }

  pub fn count_active(&self) -> i64 {
    self.cubes.values()
      .filter(|v| **v)
      .count() as i64
  }

  fn coords_to_index(&self, (x,y,z,w): Coords4D) -> i64 {
    if x.abs() > self.size + 1 || y.abs() > self.size + 1 || z.abs() > self.size + 1 || w.abs() > self.size + 1 {
      panic!("grid overflow: ({},{},{},{})", x, y, z, w);
    }
    return x * self.internal_size * self.internal_size * self.internal_size
      + y * self.internal_size * self.internal_size
      + z * self.internal_size
      + w;
  }

}