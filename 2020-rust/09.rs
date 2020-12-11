use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    let numbers = read_input("09-example.txt");
    println!("EXAMPLE\n----------");
    let invalid = part1(&numbers, 5);
    part2(&numbers, &invalid);

    let numbers = read_input("09.txt");
    println!("\nINPUT\n----------");
    let invalid = part1(&numbers, 25);
    part2(&numbers, &invalid);
  
    println!("\nCompleted in {}ms", start.elapsed().as_millis());
}

fn part1(numbers: &Vec<i64>, preamble_size: usize) -> i64 {
    let result = numbers.iter().enumerate().find(|(index, number)| {
        if index <= &preamble_size {
            return false;
        }

        let preamble = get_preamble(numbers, &index, preamble_size);
        !is_sum_of_two_numbers(number, &preamble)
    });

    let invalid = *result.unwrap().1;
    println!("PT.1: {} is invalid", invalid);
    invalid
}

fn part2(numbers: &Vec<i64>, goal: &i64) {
    // Insanely slow (10s+), using "find_map" or "map_while" could easily prevent some of the useless computations
    let result = numbers.iter().enumerate()
        .map(|(from_index, _number)| {
            let to_indexes = (from_index+1)..numbers.len();
            let result = to_indexes.into_iter()
                .map(|to_index| {
                    let slice = &numbers[from_index..to_index];
                    let slice_sum = slice.iter().sum::<i64>();
                    let slice_min = slice.iter().min().unwrap();
                    let slice_max = slice.iter().max().unwrap();
                    (slice_sum, slice_min + slice_max, to_index)
                })
                .find(|(sum, _weakness, _to_index)| sum == goal);
            if result.is_some() {
              // println!("from_index={} to_index={} sum={} weakness={}", from_index, result.unwrap().2, result.unwrap().0, result.unwrap().1);
            }
            result
        })
        .find(|result| result.is_some())
        .unwrap();
    println!("PT.2: {} is the weakness", result.unwrap().1);
}

fn get_preamble(numbers: &Vec<i64>, for_index: &usize, preamble_size: usize) -> Vec<i64> {
    let start = for_index - preamble_size - 1;
    let end = for_index;
    numbers[start..*end].to_vec()
}

fn is_sum_of_two_numbers(number: &i64, preamble: &Vec<i64>) -> bool {
    for i in preamble {
        for j in preamble {
            if i + j == *number {
                return true;
            }
        }
    }
    false
}

fn read_input(file_name: &str) -> Vec<i64> {
    fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect()
}
