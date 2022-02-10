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
    let mut oxygen = BufReader::new(
        File::open("data/day3.txt").unwrap())
        .lines()
        .filter_map(|d| usize::from_str_radix(&d.unwrap(), 2).ok())
        .collect::<Vec<_>>();

    let mut carbon = oxygen.clone();

    for idx in (0..LENGTH).rev() {
        let mut zeroes = 0;
        let mut ones = 0;

        if oxygen.len() > 1 {
            for number in &oxygen {
                let value = number & 1 << idx;
                if value > 0 { ones += 1; }
                else { zeroes += 1; }
            }
    
            if ones >= zeroes {
                oxygen.retain(|&number| number & 1 << idx > 0);
            } else {
                oxygen.retain(|&number| number & 1 << idx == 0);
            }
        }

        let mut zeroes = 0;
        let mut ones = 0;
    
        if carbon.len() > 1 {
            for number in &carbon {
                let value = number & 1 << idx;
                if value > 0 { ones += 1; }
                else { zeroes += 1; }
            }
    
            if ones >= zeroes {
                carbon.retain(|&number| number & 1 << idx == 0);
            } else {
                carbon.retain(|&number| number & 1 << idx > 0);
            }
        }
    }

    let result = oxygen[0] * carbon[0];
    println!("Day3, part2 result: {}", result);
}

