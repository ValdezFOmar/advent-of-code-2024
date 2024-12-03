use crate::{Solution, SolutionPair};
use regex::Regex;
use std::fs;

fn get_numbers(text: &str) -> Vec<i32> {
    let nums_pattern = Regex::new(r"\d+").unwrap();
    nums_pattern
        .find_iter(text)
        .map(|m| m.as_str().parse().expect("a valid usize"))
        .collect()
}

fn solution_1(input: &str) -> usize {
    fn is_safe_report(levels: &[i32]) -> bool {
        levels
            .windows(2)
            .all(|w| (1..=3).contains(&(w[0] - w[1]).abs()))
            && (levels.windows(2).all(|w| w[0] < w[1]) || levels.windows(2).all(|w| w[0] > w[1]))
    }

    input
        .lines()
        .map(get_numbers)
        .filter(|v| is_safe_report(v))
        .count()
}

fn solution_2(input: &str) -> usize {
    fn is_safe_report(_levels: &[i32]) -> bool {
        true
    }

    input
        .lines()
        .map(get_numbers)
        .filter(|v| is_safe_report(v))
        .count()
}

pub fn solve() -> SolutionPair {
    match fs::read_to_string("input/day02.txt") {
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
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn example_1() {
        assert_eq!(solution_1(EXAMPLE), 2);
    }

    #[test]
    fn example_2() {
        assert_eq!(solution_2(EXAMPLE), 4);
    }
}
