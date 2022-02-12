const BORN_AGE: usize = 9;
const CYCLE_TIME: usize = 7;
const END_TIME: usize = 80;

pub fn p1() {
    let mut fishes: [usize; BORN_AGE] = include_str!("../data/day6.txt")
        .split(',')
        .map(|n| n.trim().parse::<usize>().unwrap())
        .fold([0; BORN_AGE], |mut a, n| {
            a[n] += 1;
            a
        });

    for day in 1..END_TIME {
        fishes[(day + CYCLE_TIME) % BORN_AGE] += fishes[day % BORN_AGE];
    }

    let result = fishes.into_iter().fold(0, |sum, next| sum + next);
    println!("Day6, part1 solution: {}", result);
}

pub fn p2() {
    unimplemented!();
}
