use std::fs::File;
use std::io::{prelude::*, BufReader};

const INPUT: &str = "data/day4.txt";
const BOARD_LEN: usize = 5;

struct Board {
    rows: Vec<Vec<i32>>,
}

impl Board{
    fn new(rows: Vec<Vec<i32>>) -> Self {
        Self {
            rows,
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

    fn inside(&self, value: i32) -> bool {
        for row in &self.rows {
            if row.contains(&value) { 
                return true
            }
        }
        false
    }

    fn is_complete(&self, drawn_numbers: &Vec<i32>) -> bool {
        let mut done = true;
        for i in 0..BOARD_LEN {
            for num in self.get_row(i) {
                done &= drawn_numbers.contains(&num);
                if !done { break; }
            }
            if done { return true }
            
            done = true;
            for num in self.get_col(i) {
                done &= drawn_numbers.contains(&num);
                if !done { break; }
            }
            if done { return true }
        }
        false
    }
    
    fn sum_everything(&self, drawn_numbers: &Vec<i32>) -> i32 {
        let mut sum = 0;
        for row in &self.rows {
            for number in row {
                sum += number;
            }
        }
        for number in drawn_numbers {
            for row in &self.rows {
                if row.contains(number) { sum -= number; }
            }
        }
        sum
    }
}


pub fn p1() {
    let mut data = BufReader::new(
        File::open(INPUT).unwrap())
        .lines()
        .map(|line| line.unwrap());

    let draws = data.next()
        .map(|number|
            number.split(',')
            .map(str::to_owned)
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>());

    let rows = data
        .filter(|row| !row.is_empty())
        .map(|row| 
             row.split_whitespace()
//             .map(str::to_owned)
             .map(|nums| nums.parse::<i32>().unwrap())
             .collect::<Vec<_>>());

    let mut boards = Vec::new();
    let mut current = Vec::new();
    for row in rows {
        current.push(row);
        if current.len() == 5 {
            boards.push(Board::new(current));
            current = Vec::new();
        }
    }
    
    let mut drawn_numbers: Vec<i32> = Vec::new();
    for number in draws.unwrap() {
        drawn_numbers.push(number);
        for board in &boards {
            if board.inside(number) {
                if board.is_complete(&drawn_numbers) {
                    println!("Result is: {}", 
                             board.sum_everything(&drawn_numbers)
                             * number);
                    return
                }
            }
        }
    }

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

        let _board = Board::new(vec![
            vec0,
            vec1,
            vec2,
            vec3,
            vec4
        ]);
    }

    #[test]
    fn test_get_row() {
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
        assert_eq!(board.get_row(4), vec![21,22,23,24,25]);
    }

    #[test]
    fn test_get_col() {
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

        assert_eq!(board.get_col(0), vec![1,6,11,16,21]);
        assert_eq!(board.get_col(3), vec![4,9,14,19,24]);
    }

    #[test]
    fn test_inside() {
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

        assert_eq!(board.inside(23), true);
        assert_eq!(board.inside(26), false);
    }

    #[test]
    fn test_is_complete() {
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

        let mut drawn_numbers = Vec::new();
        assert_eq!(board.is_complete(&drawn_numbers), false);

        drawn_numbers.push(1);
        assert_eq!(board.is_complete(&drawn_numbers), false);

        drawn_numbers.append(&mut vec![2,3,4,5]);
        assert_eq!(board.is_complete(&drawn_numbers), true);

        drawn_numbers.clear();
        drawn_numbers.append(&mut vec![3, 8, 13, 18, 23]);
        assert_eq!(board.is_complete(&drawn_numbers), true);
    }

    #[test]
    fn test_sum_everything() {
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

        let mut drawn_numbers = Vec::new();
        assert_eq!(board.sum_everything(&drawn_numbers), 325);

        drawn_numbers.push(1);
        assert_eq!(board.sum_everything(&drawn_numbers), 325 - 1);

        drawn_numbers.append(&mut vec![2,3,4,5]);
        assert_eq!(board.sum_everything(&drawn_numbers), 325 - 15);

        drawn_numbers.clear();
        drawn_numbers.append(&mut vec![3, 8, 13, 18, 23]);
        assert_eq!(board.sum_everything(&drawn_numbers), 325 - 65);
    }
}
