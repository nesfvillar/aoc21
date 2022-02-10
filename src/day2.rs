use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn p1() {
    let data = BufReader::new(
        File::open("data/day2.txt").unwrap())
        .lines();

    let mut position = 0;
    let mut depth = 0;

    for line in data {
        let word = line.as_ref().unwrap().split_whitespace().collect::<Vec<&str>>();
        let direction = word[0];
        let value: i32 = word[1].parse().unwrap();

        match direction {
            "forward" => position += value,
            "backwards" => position -= value,
            "up" => depth -= value,
            "down" => depth += value,
            _ => unreachable!(),
        };
    }

    println!("Day 2, part 1 result: {}", depth * position);
}

pub fn p2() {
    let data = BufReader::new(
        File::open("data/day2.txt").unwrap())
        .lines();

    let mut position = 0;
    let mut aim = 0;
    let mut depth = 0;

    for line in data {
        let word = line.as_ref().unwrap().split_whitespace().collect::<Vec<&str>>();
        let direction = word[0];
        let value: i32 = word[1].parse().unwrap();

        match direction {
            "forward" => {
                depth += aim * value;
                position += value;
            },
            "backwards" => {
                depth -= aim * value;
                position -= value;
            },
            "up" => aim -= value,
            "down" => aim += value,
            _ => unreachable!(),
        };
    }

    println!("Day 2, part 2 result: {}", depth * position);
}
