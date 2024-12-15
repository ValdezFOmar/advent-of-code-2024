use crate::{Solution, SolutionPair};
use regex::Regex;
use std::fs;
use std::sync::LazyLock;

static NUMBERS_PATTERN: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());

fn solution_1(input: &str) -> i64 {
    NUMBERS_PATTERN
        .captures_iter(input)
        .map(|caps| {
            let (_, [left, right]) = caps.extract();
            let num1 = left.parse::<i64>().unwrap();
            let num2 = right.parse::<i64>().unwrap();
            num1 * num2
        })
        .sum()
}

fn solution_2(input: &str) -> i64 {
    let dont_instruction = "don't()";
    let do_instruction = "do()";

    let mut text = input;
    let mut enable = true;
    let mut total = 0;

    loop {
        let instruction = if enable {
            dont_instruction
        } else {
            do_instruction
        };

        let Some(start) = text.find(instruction) else {
            if enable {
                total += solution_1(text);
            }
            break;
        };

        if enable {
            total += solution_1(&text[..start]);
        }

        text = &text[start + instruction.len()..];
        enable = !enable;
    }

    total
}

pub fn solve() -> SolutionPair {
    match fs::read_to_string("input/day03.txt") {
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

    const EXAMPLE_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const EXAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn example_1() {
        assert_eq!(solution_1(EXAMPLE_1), 161);
    }

    #[test]
    fn example_2() {
        assert_eq!(solution_2(EXAMPLE_2), 48);
    }
}
