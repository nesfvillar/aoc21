use std::fs::File;
use std::io::{prelude::*, BufReader};

fn day1_p1() -> i32 {
    let mut data = BufReader::new(
        File::open("data/day1.txt").unwrap())
        .lines()
        .filter_map(|d| d.unwrap().parse::<i32>().ok());

    let mut result = 0;
    let mut first = data.next().unwrap();
    for second in data {
        if second > first { result += 1; }
        first = second;
    }

    result
}

fn day1_p2() -> i32 {
    let mut data = BufReader::new(
        File::open("data/day1.txt").unwrap())
        .lines()
        .filter_map(|d| d.unwrap().parse::<i32>().ok());

    let mut result = 0;
    let mut first = data.next().unwrap();
    let mut second = data.next().unwrap();
    let mut third = data.next().unwrap();
    for fourth in data {
        if fourth > first { result += 1; }
        first = second;
        second = third;
        third = fourth;
    }

    result
}

fn main() {
    println!("Day 1 p1: {}", day1_p1());
    println!("Day 1 p2: {}", day1_p2());
}
