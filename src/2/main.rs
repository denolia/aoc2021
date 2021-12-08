use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line")).collect()
}

fn part1() {
    println!("Hello, this is part 1 of day 2!");

    let lines: Vec<String> = lines_from_file("src/2/input.txt");

    let mut position: i32 = 0;
    let mut depth: i32 = 0;

    for line in lines {
        let command: Vec<&str> = line.split(" ").collect();

        let value = command[1].parse::<i32>().unwrap();
        match command[0] {
            "forward" => position += value,
            "down" => depth += value,
            "up" => depth -= value,
            _ => panic!("Unexpected command {}", line)
        }
        println!("position: {} depth: {}", position, depth);
    }

    println!("result: {}", position * depth);
}

fn part2() {
    println!("Hello, this is part 2 of day 2!");

    let lines: Vec<String> = lines_from_file("src/2/input.txt");

    let mut position: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;

    for line in lines {
        let command: Vec<&str> = line.split(" ").collect();

        let value = command[1].parse::<i32>().unwrap();
        match command[0] {
            "forward" => {
                position += value;
                depth += aim * value;
            }
            "down" => aim += value,
            "up" => aim -= value,
            _ => panic!("Unexpected command {}", line)
        }
        println!("position: {} depth: {} aim: {}", position, depth, aim);
    }

    println!("result: {}", position * depth);
}

fn main() {
    part1();
    part2();
}
