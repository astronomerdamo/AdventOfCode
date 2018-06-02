use std::path::Path;
use utils;

fn load_raw_input() -> String {
    let path = Path::new("./inputs/day7.txt");
    println!("Loading inputs file: {:?}", &path);
    utils::file_utils::read_inputs(path)
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
    input
        .iter()
        .cloned()
        .cycle()
        .take(size + n)
        .skip(*n)
        .collect()
}

fn sum_captcha_matches(input: &[usize], n: &usize) -> usize {
    input
        .iter()
        .zip(shiftn(input, n).iter())
        .map(|(&a, &b)| if a == b { a } else { 0 })
        .sum()
}

fn run_a(input: &[usize]) -> usize {
    sum_captcha_matches(input, &1)
}

fn run_b(input: &[usize]) -> usize {
    let halfsize = input.len() / 2;
    sum_captcha_matches(input, &halfsize)
}

pub fn main() {
    let input: Vec<usize> = parse_input();
    println!("Day 1a: {:?}", &"N/A");
    println!("Day 1b: {:?}", &"N/A");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        assert!(false);
    }

    #[test]
    fn test_run_b() {
        assert!(false);
    }
}
