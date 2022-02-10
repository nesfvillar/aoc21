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

    println!("{}", position * depth);
}

pub fn p2() {
    unimplemented!();
}
