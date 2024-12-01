use crate::{Solution, SolutionPair};
use std::fs;

fn solution_1(_input: &str) -> i64 {
    0
}

fn solution_2(_input: &str) -> i64 {
    0
}

pub fn solve() -> SolutionPair {
    match fs::read_to_string("input/day19.txt") {
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

    const EXAMPLE: &str = "";

    #[test]
    fn example_1() {
        assert_eq!(solution_1(EXAMPLE), 0);
    }

    #[test]
    fn example_2() {
        assert_eq!(solution_2(EXAMPLE), 0);
    }
}
