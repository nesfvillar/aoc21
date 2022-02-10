use std::fs::File;
use std::io::{prelude::*, BufReader};

const LENGTH: usize = 12;

pub fn p1() {
    let data = BufReader::new(
        File::open("data/day3.txt").unwrap())
        .lines()
        .filter_map(|d| isize::from_str_radix(&d.unwrap(), 2).ok())
        .collect::<Vec<_>>();

    let mut gamma = 0;
    let mut epsilon = 0;

    for idx in 0..LENGTH {
        let mut zeroes = 0;
        let mut ones = 0;

        for number in &data {
            let value = number & 1 << idx;
            if value > 0 { ones += 1; }
            else { zeroes += 1; }
        }
        if ones > zeroes {
            gamma |= 1 << idx;
        } else {
            epsilon |= 1 << idx;
        }
    }

    let result = gamma * epsilon;
    println!("Day3, part1 result: {}", result);
}

pub fn p2() {
}

