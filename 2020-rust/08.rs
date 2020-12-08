use core::panic;
use std::fs;

fn main() {
    let commands = read_input("08-example.txt");
    println!("EXAMPLE\n----------");
    part1(&commands);
    println!("-------");
    part2(&commands);

    let commands = read_input("08.txt");
    println!("INPUT\n----------");
    part1(&commands);
    println!("-------");
    part2(&commands);
}

fn part1(commands: &Vec<Command>) -> (bool, i64) {
    let mut cursor: usize = 0;
    let mut acc: i64 = 0;
    let mut consumed_commands: Vec<usize> = Vec::new();
    let mut aborted = false;

    loop {
        let command = commands.get(cursor).unwrap();

        // println!("[acc={}] {}> {} {}", acc, cursor, command.name, command.value);
        match command.name.as_str() {
            "acc" => {
                acc += command.value;
                cursor += 1;
            }
            "jmp" => cursor = (cursor as i64 + command.value) as usize,
            "nop" => cursor += 1,
            _ => panic!("unknown command"),
        }

        if consumed_commands.contains(&cursor) {
            aborted = true;
            break;
        }
        if cursor >= commands.len() {
            break;
        }
        consumed_commands.push(cursor);
    }

    println!(
        "Acc value afer {} iterations: {}",
        consumed_commands.len(),
        acc
    );
    return (aborted, acc);
}

fn part2(commands: &Vec<Command>) {
    for index in 0..commands.len() {
        print!("{}: ", index);
        let mutated_commands = mutate(commands, index);
        let (aborted, acc) = part1(&mutated_commands);
        if !aborted {
            println!("Last mutation did not abort, result is {}", acc);
            break;
        }
    }
}

fn mutate(commands: &Vec<Command>, index: usize) -> Vec<Command> {
    let mut cloned_commands = commands.to_vec();
    let name_to_replace = get_name_at(&cloned_commands, index);
    let value = cloned_commands.get(index).unwrap().value;

    let _dropped = std::mem::replace(
        &mut cloned_commands[index],
        Command {
            name: match name_to_replace.as_str() {
                "jmp" => "nop".to_string(),
                "nop" => "jmp".to_string(),
                _ => name_to_replace.clone(),
            },
            value,
        },
    );
    print!(
        "[mutate {} into {}] ",
        name_to_replace,
        get_name_at(&cloned_commands, index)
    );

    return cloned_commands;
}

fn get_name_at(commands: &Vec<Command>, index: usize) -> String {
    return commands.get(index).unwrap().name.clone();
}

struct Command {
    name: String,
    value: i64,
}

impl Clone for Command {
    fn clone(&self) -> Self {
        return Command {
            name: self.name.to_string(),
            value: self.value,
        };
    }
}

fn read_input(file_name: &str) -> Vec<Command> {
    return fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(|line| line.split_at(4))
        .map(|(name_str, value)| Command {
            name: name_str.trim_end().to_string(),
            value: value.parse::<i64>().unwrap(),
        })
        .collect::<Vec<Command>>();
}
