pub fn p1() {
    let mut data: Vec<i32> = include_str!("../data/day7.txt")
        .split(",")
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect();

    data.sort();
    let length = data.len();

    if length % 2 == 0 {
        let median = data[length / 2];
        let result = data.into_iter().fold(0, |sum, n| sum + i32::abs(n - median));
        println!("Day7, part 1 result: {}", result);
    } else {
        panic!();
    }
}

pub fn p2() {
    let mut data: Vec<i64> = include_str!("../data/day7.txt")
        .split(",")
        .map(|s| s.trim().parse::<i64>().unwrap())
        .collect();

    data.sort();
    let mut result = i64::MAX;

    for val in 0..*data.last().unwrap() {
        result = (data.clone().into_iter().fold(0, |sum, n| sum + (val - n).pow(2) + i64::abs(val - n)) / 2).min(result);
    }
    println!("Day7, part 2 result: {}", result);
}
