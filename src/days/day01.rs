use crate::{Solution, SolutionPair};
use std::collections::HashMap;
use std::fs;

fn number_lists(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut left_column = Vec::new();
    let mut right_column = Vec::new();

    for line in input.lines() {
        let mut parts = line.split_whitespace();
        let left = parts.next().expect("a left number");
        let right = parts.next().expect("a right number");

        left_column.push(left.parse::<i64>().expect("a valid number"));
        right_column.push(right.parse::<i64>().expect("a valid number"));
    }

    (left_column, right_column)
}

fn solution_1(input: &str) -> i64 {
    let (mut left, mut right) = number_lists(input);

    left.sort();
    right.sort();

    left.iter()
        .zip(right)
        .map(|pair| (pair.0 - pair.1).abs())
        .sum()
}

fn solution_2(input: &str) -> i64 {
    let (left, right) = number_lists(input);
    let mut counts = HashMap::new();

    for number in right {
        let new_count = match counts.get(&number) {
            Some(count) => count + 1,
            None => 1,
        };
        counts.insert(number, new_count);
    }

    left.iter()
        .map(|num| match counts.get(num) {
            Some(count) => num * count,
            None => 0,
        })
        .sum()
}

pub fn solve() -> SolutionPair {
    match fs::read_to_string("input/day01.txt") {
        Ok(input) => {
            let sol1 = solution_1(&input);
            let sol2 = solution_2(&input);
            (Solution::from(sol1), Solution::from(sol2))
        }
        Err(_) => (Solution::from(0), Solution::from(0)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn example_1() {
        assert_eq!(solution_1(EXAMPLE), 11);
    }

    #[test]
    fn example_2() {
        assert_eq!(solution_2(EXAMPLE), 31);
    }
}
