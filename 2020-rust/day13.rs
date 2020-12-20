/**
 * Alternate implementations:
 * - Using enums, #[derive] and custom implems of standard traits: https://github.com/ttencate/aoc2020/blob/master/src/bin/11.rs
 */

use std::{fmt::Display, fs};
use std::time::Instant;
use bigdecimal::BigDecimal;

static X: i64 = -1;

fn main() {
    let start = Instant::now();

    let input = read_input("13-example.txt");
    println!("EXAMPLE\n----------");
    part1(&input); // 295
    part2(&input); // 1068781
    part2(&(0, read_buses("17,x,13,19"))); // 3417
    part2(&(0, read_buses("67,7,59,61"))); // 754018
    part2(&(0, read_buses("67,x,7,59,61"))); // 779210
    part2(&(0, read_buses("67,7,x,59,61"))); // 1261476
    part2(&(0, read_buses("1789,37,47,1889"))); // 1202161486

    let input = read_input("13.txt");
    println!("\nINPUT\n----------");
    part1(&input);
    part2(&input);

    println!("\nCompleted in {}ms", start.elapsed().as_millis());
}

fn part1(input: &Input) {
    let min_delay = input.0;
    let max_delay:i64 = min_delay + input.1.iter().max().unwrap();
    for delay in min_delay..max_delay {
        let matching_bus = input.1.iter()
            .filter(|bus| **bus != X).
            find(|bus| {
                let already_departed: f64 = 1. * (delay as f64) / (**bus as f64);
                already_departed == already_departed.round()
            });
        if matching_bus.is_some() {
            println!("PT.1: bus={},result={}", matching_bus.unwrap(), matching_bus.unwrap()*(delay - min_delay));
            break;
        }
    }
}

fn part2(input: &Input) {
    let not_found = BigDecimal::from(-1);
    let bus_offsets = input.1.iter().enumerate()
        .filter(|(_index, bus)| **bus != -1)
        .map(|(index, bus)| BusOffset { id: BigDecimal::from(*bus), offset: BigDecimal::from(index as i64) })
        .collect::<Vec<BusOffset>>();
    let (first, others) = bus_offsets.split_at(1);   
    
    let mut acc = BusOffset { id: first[0].id.clone(), offset: first[0].offset.clone() };
    // println!("{}", acc);
    for bus_offset in others {
        let mut time = acc.offset.clone();
        let mut first_match = not_found.clone();
        loop {
            time = time.clone() + acc.id.clone();
            let bus_progress = (time.clone() + bus_offset.offset.clone()) / bus_offset.id.clone();
            let bus_matches = bus_progress.is_integer();
            if bus_matches {
                if first_match == not_found {
                    first_match = time.clone();
                } else {
                    acc = BusOffset {
                        id: time.clone() - first_match.clone(),
                        offset: first_match.clone()
                    };
                    // println!("{}\n  > {}", bus_offset, acc);
                    break;
                }
            }
        }
    }
    println!("PT.2: {}", acc.offset);
}

type Input = (i64, Vec<i64>);

struct BusOffset {
    id: BigDecimal,
    offset: BigDecimal
}

impl Display for BusOffset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BusOffset[every {}, will depart in {}]", self.id, self.offset)
    }
}

fn read_input(file_name: &str) -> Input {
    let raw_str = fs::read_to_string(file_name).unwrap_or(String::new());
    let mut raw_strings = raw_str.lines().map(|s| s.to_string());
    let min_delay = raw_strings.next().unwrap().parse().unwrap();
    let buses = read_buses(&raw_strings.next().unwrap());
    (min_delay, buses)
}

fn read_buses(input: &str) -> Vec<i64> {
    // println!("\n{}", input);
    return input.split(",").map(|token| token.parse::<i64>().unwrap_or(-1)).collect::<Vec<i64>>();
}
