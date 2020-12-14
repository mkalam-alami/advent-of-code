/**
 * Alternate implementations:
 * - Using enums, #[derive] and custom implems of standard traits: https://github.com/ttencate/aoc2020/blob/master/src/bin/11.rs
 */

use std::fs;
use std::time::Instant;

static DIRECTIONS: [(i64, i64); 8] = [(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)];

fn main() {
    let start = Instant::now();

    let boat = read_input("11-example.txt");
    println!("EXAMPLE\n----------");
    run(&boat, &update_seat_pt1);
    run(&boat, &update_seat_pt2);

    let boat = read_input("11.txt");
    println!("\nINPUT\n----------");
    run(&boat, &update_seat_pt1);
    run(&boat, &update_seat_pt2);

    println!("\nCompleted in {}ms", start.elapsed().as_millis());
}

fn run(boat: &Boat, update_seat: &SeatUpdater) {
    let mut state = update_boat(boat, update_seat);
    while state.has_changed {
        state = update_boat(&state.boat, update_seat);
    }
    println!("Occupied seats: {}", state.boat.count_occupied_seats());
}

fn update_boat(boat: &Boat, update_seat: &SeatUpdater) -> BoatUpdateResult {
    let mut has_changed = false;
    let seats: Vec<Vec<char>> = (0..boat.height())
        .map(|y| {
            (0..boat.width())
            .map(|x| {
                let old_char = boat.get_seat(x, y);
                let new_char = update_seat(boat, &x, &y);
                has_changed = has_changed || old_char != new_char;
                new_char
            })
            .collect()
        })
        .collect();

    BoatUpdateResult {
        has_changed,
        boat: Boat { seats }
    }
}

fn update_seat_pt1(boat: &Boat, x: &i64, y: &i64) -> char {
    match boat.get_seat(*x, *y) {
        'L' => if boat.count_visible_taken_seats(x, y, false) == 0 { '#' } else { 'L' }
        '#' => if boat.count_visible_taken_seats(x, y, false) >= 4 { 'L' } else { '#' }
        _ => '.'
    }
}

fn update_seat_pt2(boat: &Boat, x: &i64, y: &i64) -> char {
    match boat.get_seat(*x, *y) {
        'L' => if boat.count_visible_taken_seats(x, y, true) == 0 { '#' } else { 'L' }
        '#' => if boat.count_visible_taken_seats(x, y, true) >= 5 { 'L' } else { '#' }
        _ => '.'
    }
}

struct Boat {
    seats: Vec<Vec<char>>
}

impl Boat {
    fn height(&self) -> i64 {
        self.seats.len() as i64
    }
    fn width(&self) -> i64 {
        self.seats.get(0).map(|row| row.len()).unwrap_or(0) as i64
    }
    fn get_seat(&self, x: i64, y: i64) -> char {
        *self.seats.get(y as usize)
            .and_then(|row| row.get(x as usize))
            .unwrap_or(&' ')
    }
    fn count_visible_taken_seats(&self, around_x: &i64, around_y: &i64, see_beyond_neighbors: bool) -> i64 {
        DIRECTIONS.iter()
            .map(|(dx, dy)| (around_x + dx, around_y + dy, dx, dy))
            .filter(|(x, y, _dx, _dy)| x != around_x || y != around_y)
            .map(|(mut x, mut y, dx, dy)| {
                let mut seen_seat = self.get_seat(x, y);
                if see_beyond_neighbors {
                    while seen_seat == '.' {
                        x += dx;
                        y += dy;
                        seen_seat = self.get_seat(x, y);
                    }   
                }
                (seen_seat == '#') as i64
            })
            .sum::<i64>()
    }
    fn count_occupied_seats(&self) -> i64 {
        self.seats
            .iter()
            .map(|l| l.iter().filter(|c| *c == &'#').count() as i64)
            .sum()
    }
}

struct BoatUpdateResult {
    has_changed: bool,
    boat: Boat
}

type SeatUpdater = dyn Fn(&Boat, &i64, &i64) -> char;

fn read_input(file_name: &str) -> Boat {
    let seats = fs::read_to_string(file_name)
        .unwrap_or(String::new())
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    Boat { seats }
}
