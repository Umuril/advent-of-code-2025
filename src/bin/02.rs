use itertools::Itertools;
use nom::{
    IResult, Parser, bytes::complete::tag, multi::separated_list1, sequence::separated_pair,
};
use phf::Map;

advent_of_code::solution!(2);

static MAP: Map<u32, &[u64]> = phf::phf_map! {
    1 => &[],
    2 => &[11],
    3 => &[111],
    4 => &[1111, 0101],
    5 => &[11111],
    6 => &[111111, 010101, 001001],
    7 => &[1111111],
    8 => &[11111111, 01010101, 00010001],
    9 => &[111111111, 001001001],
    10 => &[1111111111, 0101010101, 0000100001],
};

pub fn parse_pair(input: &str) -> IResult<&str, (u64, u64)> {
    separated_pair(
        nom::character::complete::u64,
        nom::character::char('-'),
        nom::character::complete::u64,
    )
    .parse(input)
}

pub fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let result: IResult<&str, Vec<(u64, u64)>> = separated_list1(tag(","), parse_pair).parse(input);

    result.expect("Correct input").1
}

pub fn part_one(input: &str) -> Option<String> {
    let ranges = parse_input(input);
    let mut acc = 0u64;

    for range in ranges {
        let digits0 = range.0.checked_ilog10().unwrap_or(0) + 1;
        let digits1 = range.1.checked_ilog10().unwrap_or(0) + 1;

        for digits in digits0..=digits1 {
            if digits % 2 == 1 {
                continue;
            }
            let divisor = 10u64.pow(digits / 2) + 1;
            for n in range.0..=range.1 {
                if n.checked_ilog10().unwrap_or(0) + 1 != digits {
                    continue;
                }
                if n % divisor == 0 {
                    // println!(
                    //     "Found {n} in {} - {} with divisor {divisor} and digits {digits}",
                    //     range.0, range.1
                    // );
                    acc += n;
                }
            }
        }
    }
    Some(acc.to_string())
}

pub fn _is_a_repeated_number_v1(n: u64) -> bool {
    let n = n.to_string();
    for i in 1..n.len() {
        let v = n
            .chars()
            .collect::<Vec<char>>()
            .chunks(i)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<String>>();

        if v.iter().all_equal() {
            // println!("{v:?}");
            return true;
        }
    }

    false
}

pub fn is_a_repeated_number_v2(n: u64) -> bool {
    let digits = n.checked_ilog10().unwrap_or(0) + 1;

    let masks = MAP.get(&digits).unwrap();

    for mask in masks.iter() {
        if n % mask == 0 {
            return true;
        }
    }

    false
}

pub fn part_two(input: &str) -> Option<String> {
    let ranges = parse_input(input);
    let mut acc = 0u64;

    for range in ranges {
        for n in range.0..=range.1 {
            if is_a_repeated_number_v2(n) {
                // println!("Found {n} in {} - {}", range.0, range.1);
                acc += n;
            }
        }
    }
    Some(acc.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("1227775554".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4174379265".to_string()));
    }
}
