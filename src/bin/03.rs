use itertools::Itertools;
use nom::{
    IResult, Parser,
    character::complete::{digit1, newline},
    multi::separated_list1,
};

advent_of_code::solution!(3);

pub fn parse_input(input: &str) -> Vec<&str> {
    let result: IResult<&str, Vec<&str>> = separated_list1(newline, digit1).parse(input);

    result.expect("Parse input").1
}

pub fn find_max(nums: &str, size: isize) -> u64 {
    let num = nums
        .chars()
        .map(|n| n.to_digit(10).unwrap() as u64)
        .collect_vec();
    let mut start = 0;
    let mut to_skip = size - 1;
    let mut acc = 0u64;

    for _ in 0..size {
        let m = num
            .iter()
            .skip(start)
            .rev()
            .skip(to_skip as usize)
            .rev()
            .max()
            .unwrap();
        let p = num
            .iter()
            .skip(start)
            .find_position(|&&x| x == *m)
            .unwrap()
            .0;

        acc = acc * 10 + m;

        // println!("NUM: {nums} - M: {m} - P: {p} - START: {start} - SKIP: {to_skip} - ACC: {acc}");

        start = start + p + 1;
        to_skip -= 1;
    }

    acc
}

pub fn part_one(input: &str) -> Option<String> {
    let nums = parse_input(input);
    let mut acc = 0;

    for nums in nums {
        let num = nums.chars().map(|n| n.to_digit(10).unwrap()).collect_vec();
        let m1 = num
            .iter()
            .enumerate()
            .rev()
            .skip(1)
            .rev()
            .max_by_key(|x| x.1 * 1000 - x.0 as u32)
            .unwrap();
        let m2 = num.iter().skip(m1.0 + 1).max().unwrap();

        acc += m1.1 * 10 + m2;
    }

    Some(acc.to_string())
}

pub fn part_two(input: &str) -> Option<String> {
    let nums = parse_input(input);
    let mut acc = 0;

    for nums in nums {
        let m = find_max(nums, 12);

        acc += m;
    }

    Some(acc.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("357".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("3121910778619".to_string()));
    }
}
