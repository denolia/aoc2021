use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|x| x.parse::<i32>().unwrap()).collect()
}

fn part1() {
    println!("Hello, this is part 1 of day 1!");

    let lines: Vec<i32> = lines_from_file("src/1/input.txt");

    let mut count: i32 = 0;
    let mut prev: i32 = -1;

    for line in lines {
        if prev >= 0 &&  prev < line {
            count = count + 1;
            println!("{}: increased because it is greater than {}", line, prev);
        }
        prev = line;
    }

    println!("result: {}", count);
}

fn part2() {
    println!("Hello, this is part 2 of day 1!");

    let lines: Vec<i32> = lines_from_file("src/1/input.txt");

    let mut count: i32 = 0;
    let mut prev_a: i32 = -1;
    let mut prev_b: i32 = -1;
    let mut prev_c: i32 = -1;

    for current in lines {
        if prev_a >= 0 && prev_b >= 0 && prev_c >= 0 {
            if (prev_b + prev_a + prev_c) < (prev_a + prev_b + current) {
                count = count + 1;
                println!("{}: increased {} ", current, count );
            }
        }
        prev_c = prev_b;
        prev_b = prev_a;
        prev_a = current;
    }

    println!("result: {}", count);
}

fn main(){
    part1();
    part2();
}
