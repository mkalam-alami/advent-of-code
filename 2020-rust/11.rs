use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let boat = read_input("11-example.txt");
    println!("EXAMPLE\n----------");
    part1(&boat);

    let boat = read_input("11.txt");
    println!("\nINPUT\n----------");
    part1(&boat);

    println!("Completed in {}ms", start.elapsed().as_millis());
}

fn part1(boat: &Boat) {
    let mut state = boat.clone();
    let mut iteration_count = 0;

    loop {
        iteration_count += 1;

        let result = iteration(&state, &iteration_cell_pt1);
        let state_has_changed = result.0;
        state = result.1;
        
        if !state_has_changed {
            break;
        }
        if iteration_count % 10 == 0 {
            println!("iteration {}...", iteration_count);
        }
    }

    println!("PT.1: {}", state.count_occupied());
}

fn iteration(boat: &Boat, iteration_cell: &dyn Fn(&Boat, &i64, &i64) -> char) -> (bool, Boat) {
    let mut has_changed = false;
    let seats: Vec<String> = (0..boat.height)
        .map(|y| {
            (0..boat.width)
                .map(|x| {
                    let old_char = boat.get_seat(&x, &y);
                    let new_char = iteration_cell(boat, &x, &y);
                    has_changed = has_changed || old_char != new_char;
                    new_char
                })
                .collect()
        })
        .collect();

    // seats.iter().for_each(|l| println!("{}", l));
    // println!("------------");

    (has_changed, Boat {
        seats,
        width: boat.width,
        height: boat.height,
    })
}

fn iteration_cell_pt1(boat: &Boat, x: &i64, y: &i64) -> char {
    match boat.get_seat(x, y) {
        'L' => {
            if count_around(boat, x, y, '#') == 0 {
                '#'
            } else {
                'L'
            }
        }
        '#' => {
            if count_around(boat, x, y, '#') >= 4 {
                'L'
            } else {
                '#'
            }
        }
        _ => '.',
    }
}

fn count_around(boat: &Boat, x: &i64, y: &i64, token: char) -> i64 {
    let directions: [(i64, i64); 8] = [(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)];
    directions.iter()
        .map(|(dx, dy)| {
            let itx = x + dx;
            let ity = y + dy;
            if itx < 0
                || itx >= boat.width
                || ity < 0
                || ity >= boat.height
                || (&itx == x && &ity == y)
            {
                return 0;
            } else {
                // println!("{} {} {}", itx, ity, boat.get_seat(itx, ity));
                return (boat.get_seat(&itx, &ity) == token) as i64;
            }
        })
        .sum::<i64>()
}

struct Boat {
    seats: Vec<String>,
    height: i64,
    width: i64,
}

impl Boat {
    fn get_seat(&self, x: &i64, y: &i64) -> char {
        self.seats[*y as usize].chars().nth(*x as usize).unwrap()
    }
    fn count_occupied(&self) -> i64 {
        self.seats
            .iter()
            .map(|l| l.chars().filter(|c| c == &'#').count() as i64)
            .sum()
    }
    fn clone(&self) -> Boat {
        Boat {
            seats: self.seats.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

fn read_input(file_name: &str) -> Boat {
    let seats = fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|l| l.to_owned())
        .collect::<Vec<String>>();
    Boat {
        width: seats[0].len() as i64,
        height: seats.len() as i64,
        seats,
    }
}
