use std::env;
use utils;

fn load_raw_input() -> String {
    utils::ifile::read_inputs(
        &env::args().nth(1).expect("FAILURE : FILE PATH")
    )
}

fn parse_input() -> Vec<usize> {
    load_raw_input()
        .trim()
        .split("")
        .filter(|c| !c.is_empty())
        .map(|c| c.parse::<usize>().unwrap())
        .collect()
}

fn shiftn(input: &[usize], n: &usize) -> Vec<usize> {
    let size = input.len();
    input.iter().cloned().cycle().take(size + n).skip(*n).collect()
}

fn sum_captcha_matches(input: &[usize], n: &usize) -> usize {
    input.iter()
        .zip(shiftn(input, n).iter())
        .map(|(&a, &b)| {if a == b {a} else {0}} )
        .sum()
}

fn run_a(input: &[usize]) -> usize {
    sum_captcha_matches(&input, &1)
}

fn run_b(input: &[usize]) -> usize {
    let halfsize = input.len()/2;
    sum_captcha_matches(&input, &halfsize)
}

/// Day 1: Inverse Captcha
pub fn main() -> () {
    let input: Vec<usize> = parse_input();
    println!("Day 1a: {:?}", &run_a(&input));
    println!("Day 1b: {:?}", &run_b(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        assert_eq!(run_a(&vec![1, 1, 2, 2]), 3);
        assert_eq!(run_a(&vec![1, 1, 1, 1]), 4);
        assert_eq!(run_a(&vec![1, 2, 3, 4]), 0);
        assert_eq!(run_a(&vec![9, 1, 2, 1, 2, 1, 2, 9]), 9);
    }

    #[test]
    fn test_run_b() {
        assert_eq!(run_b(&vec![1, 2, 1, 2]), 6);
        assert_eq!(run_b(&vec![1, 2, 2, 1]), 0);
        assert_eq!(run_b(&vec![1, 2, 3, 4, 2, 5]), 4);
        assert_eq!(run_b(&vec![1, 2, 3, 1, 2, 3]), 12);
        assert_eq!(run_b(&vec![1, 2, 1, 3, 1, 4, 1, 5]), 4);
    }
}
