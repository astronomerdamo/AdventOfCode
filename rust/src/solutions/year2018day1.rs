use std::path::Path;
use utils;

fn load_raw_input() -> String {
    let path = Path::new("./inputs/year2018day1.txt");
    println!("Loading inputs file: {:?}", &path);
    utils::file_utils::read_inputs(path)
}

fn parse_input() -> Vec<isize> {
    load_raw_input()
        .lines()
        .map(|c| c.parse::<isize>().unwrap())
        .collect()
}

fn run_a(input: &[isize]) -> isize {
    input
        .iter()
        .sum()
}

pub fn main() {
    let input: Vec<isize> = parse_input();
    println!("Year 2018 Day 1a: {:?}", &run_a(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_a() {
        assert_eq!(run_a(&vec![1, 1, 1]), 3);
        assert_eq!(run_a(&vec![1, 1, -2]), 0);
        assert_eq!(run_a(&vec![-1, -2, -3]), -6);
        assert_eq!(run_a(&vec![1, -2, 3, 1]), 3);
    }
}