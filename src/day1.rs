use std::fs::File;
use std::io::{prelude::*, BufReader};

pub fn p1() {
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
    
    println!("Day 1, part 1 result: {}", result);
}

pub fn p2() {
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
    
    println!("Day1, part 2 result: {}", result);
}

