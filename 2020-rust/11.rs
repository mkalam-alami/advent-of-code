use std::fs;

fn main() {
    let boat = read_input("11-example.txt");
    println!("EXAMPLE\n----------");
    part1(&boat);

    let boat = read_input("11.txt");
    println!("\nINPUT\n----------");
    part1(&boat);
}

fn part1(boat: &Boat) {
    let mut state = boat.clone();
    let mut hash = String::new();
    loop {
        state = iteration(&state);
        // state.print();

        let old_hash = hash;
        hash = state.hash();
        if hash == old_hash {
            break;
        }
    }

    println!("PT.1: {}", state.count_occupied());
}

fn iteration(boat: &Boat) -> Boat {
    let seats: Vec<String> = (0..boat.height)
        .map(|y| {
            (0..boat.width)
                .map(|x| iteration_cell(boat, x, y))
                .collect()
        })
        .collect();
    Boat {
        seats,
        width: boat.width,
        height: boat.height,
    }
}

fn iteration_cell(boat: &Boat, x: i64, y: i64) -> char {
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

fn count_around(boat: &Boat, x: i64, y: i64, token: char) -> i64 {
    (y - 1..y + 2)
        .map(|ity| {
            (x - 1..x + 2)
                .map(|itx| {
                    if itx < 0
                        || itx >= boat.width
                        || ity < 0
                        || ity >= boat.height
                        || (itx == x && ity == y)
                    {
                        return 0;
                    } else {
                        // println!("{} {} {}", itx, ity, boat.get_seat(itx, ity));
                        return (boat.get_seat(itx, ity) == token) as i64;
                    }
                })
                .sum::<i64>()
        })
        .sum()
}

struct Boat {
    seats: Vec<String>,
    height: i64,
    width: i64,
}

impl Boat {
    fn get_seat(&self, x: i64, y: i64) -> char {
        self.seats[y as usize].chars().nth(x as usize).unwrap()
    }
    fn count_occupied(&self) -> i64 {
        self.seats
            .iter()
            .map(|l| l.chars().filter(|c| c == &'#').count() as i64)
            .sum()
    }
    fn hash(&self) -> String {
        self.seats.join("\n")
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
