use std::fs::File;
use std::io::{prelude::*, BufReader};

const INPUT: &str = "data/day4.txt";
const BOARD_LEN: usize = 5;

struct Board {
    rows: Vec<Vec<i32>>,
    sum_matches: i32,
}

impl Board{
    fn new(rows: Vec<Vec<i32>>) -> Self {
        Self {
            rows,
            sum_matches: 0,
        }
    }
    fn get_row(&self, i: usize) -> Vec<i32> {
        self.rows[i].clone()
    }

    fn get_col(&self, j: usize) -> Vec<i32> {
        let mut res = Vec::with_capacity(BOARD_LEN);
        for i in 0..BOARD_LEN {
            res.push(self.rows[i][j])
        }
        res
    }
}

pub fn p1() {
    let mut data = BufReader::new(
        File::open(INPUT).unwrap())
        .lines()
        .map(|line| line.unwrap());

    let draws = data.next()
        .unwrap()
        .split(',')
        .filter_map(|n| n.parse::<i32>().ok());

    let rows = data
        .map(|row| 
             row.split_whitespace()
             .map(str::to_owned)
             .map(|nums| nums.parse::<i32>().unwrap())
             .collect::<Vec<_>>());
}

pub fn p2() {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use crate::day4::*;

    #[test]
    fn test_board() {
        let vec0 = vec![1,2,3,4,5];
        let vec1 = vec![6,7,8,9,10];
        let vec2 = vec![11,12,13,14,15];
        let vec3 = vec![16,17,18,19,20];
        let vec4 = vec![21,22,23,24,25];

        let board = Board::new(vec![
            vec0,
            vec1,
            vec2,
            vec3,
            vec4
        ]);

        assert_eq!(board.get_row(0), vec![1,2,3,4,5]);
        assert_eq!(board.get_col(0), vec![1,6,11,16,21]);
        assert_eq!(board.get_row(4), vec![21,22,23,24,25]);
        assert_eq!(board.get_col(3), vec![4,9,14,19,24]);
    }
}
