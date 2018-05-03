use std::path::Path;
use utils;

fn load_raw_input() -> String {
    let path = Path::new("./inputs/day2.txt");
    println!("Loading inputs file: {:?}", &path);
    utils::file_utils::read_inputs(path)
}

fn parse_input() -> Vec<Vec<usize>> {
    load_raw_input()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|c| c.parse::<usize>().unwrap())
                .collect()
        })
        .collect()
}

fn even_div_values_from_slice(s: &[usize]) -> usize {
    s.iter()
        .map(|x| {
            s.iter()
                .map(|y| if x != y && x % y == 0 { x / y } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn run_a(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .fold(0, |acc, s| acc + s.iter().max().unwrap() - s.iter().min().unwrap())
}

fn run_b(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .fold(0, |acc, s| acc + even_div_values_from_slice(&s))
}

/// Day 2: Corruption Checksum
pub fn main() {
    let input: Vec<Vec<usize>> = parse_input();
    println!("Day 2a: {:?}", &run_a(&input));
    println!("Day 2b: {:?}", &run_b(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        let test_input = vec![vec![5, 1, 9, 5], vec![7, 5, 3], vec![2, 4, 6, 8]];
        assert_eq!(run_a(&test_input), 18);
    }

    #[test]
    fn test_run_b() {
        let test_input = vec![vec![5, 9, 2, 8], vec![9, 4, 7, 3], vec![3, 8, 6, 5]];
        assert_eq!(run_b(&test_input), 9);
    }
}
