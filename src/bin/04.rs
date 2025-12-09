use advent_of_code::{ALL_8_POINTS, Matrix};

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<String> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();
    let matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let mut acc = 0u64;

    for point in matrix.as_points() {
        if !matrix.get(&point).is_some_and(|&x| x == b'@') {
            continue;
        }
        let mut rolls = 0u32;
        for d in ALL_8_POINTS.iter() {
            let p = matrix.get(&(point + d));
            if p.is_some_and(|&x| x == b'@') {
                rolls += 1;
            }
        }
        if rolls < 4 {
            acc += 1;
        }
    }

    // println!("{matrix}");

    Some(acc.to_string())
}

pub fn part_two(input: &str) -> Option<String> {
    let rows = input.trim().split('\n').collect::<Vec<&str>>();
    let mut matrix = Matrix::from(rows.len(), rows.len(), rows.join("").into());

    let mut acc = 0u64;
    let mut old_acc = 1u64;

    while old_acc != acc {
        old_acc = acc;

        for point in matrix.as_points() {
            if !matrix.get(&point).is_some_and(|&x| x == b'@') {
                continue;
            }
            let mut rolls = 0u32;
            for d in ALL_8_POINTS.iter() {
                let p = matrix.get(&(point + d));
                if p.is_some_and(|&x| x == b'@') {
                    rolls += 1;
                }
            }
            if rolls < 4 {
                matrix.update(&point, b'.');
                acc += 1;
            }
        }

        // println!("{matrix}\n{acc}\n");
    }

    Some(acc.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("13".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("43".to_string()));
    }
}
