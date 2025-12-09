use std::fmt::Debug;

use nom::{
    IResult, Parser, branch::alt, bytes::complete::tag, character::complete::newline,
    character::complete::u32, combinator::map, multi::separated_list1, sequence::pair,
};
advent_of_code::solution!(1);

pub enum Direction {
    Left,
    Right,
}

impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "L"),
            Self::Right => write!(f, "R"),
        }
    }
}

pub fn parse_input_one(input: &str) -> Vec<(Direction, u32)> {
    let result: IResult<&str, Vec<(Direction, u32)>> = separated_list1(
        newline,
        pair(
            alt((
                map(tag("L"), |_| Direction::Left),
                map(tag("R"), |_| Direction::Right),
            )),
            u32,
        ),
    )
    .parse(input);

    result.expect("Error parsing input").1
}

pub fn part_one(input: &str) -> Option<String> {
    let numbers = parse_input_one(input);
    let mut start = 50i32;
    let mut acc = 0u64;
    for (d, num) in numbers {
        let shift = match d {
            Direction::Left => -1,
            Direction::Right => 1,
        } * num as i32;
        start = (start + shift) % 100;
        if start == 0 {
            acc += 1;
        }
    }
    Some(acc.to_string())
}

pub fn part_two(input: &str) -> Option<String> {
    let numbers = parse_input_one(input);
    let mut start = 50i32;
    let mut acc = 0u64;
    let mut acc2;

    for (d, num) in numbers {
        let num = num as i32;
        let shift = match d {
            Direction::Left => -num,
            Direction::Right => num,
        };

        // print!("START {start:2} / ");

        (start, acc2) = rotations(start, shift);

        acc += acc2;

        // println!("{d:?}{num:02} => SHIFT {shift:3} / ROT {acc2:2} / END {start:2} / ACC {acc:2}");
    }

    Some(acc.to_string())
}

fn rotations(from: i32, amount: i32) -> (i32, u64) {
    let end = ((from + amount).rem_euclid(100)).abs();

    let mut rot;
    if from == 0 {
        rot = amount.abs().div_euclid(100);
    } else {
        rot = (from + amount).div_euclid(100).abs();
        if end == 0 && amount < 0 {
            rot += 1;
        }
    }

    (end, rot as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("3".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("6".to_string()));
    }

    #[test]
    fn test_rotations() {
        assert_eq!(
            rotations(0, 0),
            (0, 0),
            "We are testing from {} - shift {}",
            0,
            0
        );
        assert_eq!(
            rotations(0, 100),
            (0, 1),
            "We are testing from {} - shift {}",
            0,
            100
        );
        assert_eq!(
            rotations(0, -100),
            (0, 1),
            "We are testing from {} - shift {}",
            0,
            -100
        );
        assert_eq!(
            rotations(0, 1),
            (1, 0),
            "We are testing from {} - shift {}",
            0,
            1
        );
        assert_eq!(
            rotations(0, -1),
            (99, 0),
            "We are testing from {} - shift {}",
            0,
            -1
        );
        assert_eq!(
            rotations(0, 101),
            (1, 1),
            "We are testing from {} - shift {}",
            0,
            101
        );
        assert_eq!(
            rotations(0, -101),
            (99, 1),
            "We are testing from {} - shift {}",
            0,
            -101
        );
        assert_eq!(
            rotations(0, 200),
            (0, 2),
            "We are testing from {} - shift {}",
            0,
            200
        );
        assert_eq!(
            rotations(0, -200),
            (0, 2),
            "We are testing from {} - shift {}",
            0,
            -200
        );
        assert_eq!(
            rotations(0, 201),
            (1, 2),
            "We are testing from {} - shift {}",
            0,
            201
        );
        assert_eq!(
            rotations(0, -201),
            (99, 2),
            "We are testing from {} - shift {}",
            0,
            -201
        );
        assert_eq!(
            rotations(55, -55),
            (0, 1),
            "We are testing from {} - shift {}",
            55,
            -55
        );
    }
}
