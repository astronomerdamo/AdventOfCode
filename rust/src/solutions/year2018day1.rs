use std::collections::HashSet;
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
    input.iter().sum()
}

fn run_b(input: &[isize]) -> isize {
    let mut frequencies_seen = HashSet::new();
    let mut index: usize = 0;
    let mut frequency_sum: isize = 0;
    frequencies_seen.insert(frequency_sum);
    loop {
        frequency_sum += input.get(index % input.len()).unwrap();
        if frequencies_seen.contains(&frequency_sum) {
            break;
        } else {
            frequencies_seen.insert(frequency_sum);
            index += 1;
        }
    }
    frequency_sum
}

pub fn main() {
    let input: Vec<isize> = parse_input();
    println!("Year 2018 Day 1a: {:?}", &run_a(&input));
    println!("Year 2018 Day 2a: {:?}", &run_b(&input));
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

    #[test]
    fn test_run_b() {
        assert_eq!(run_b(&vec![1, -1]), 0);
        assert_eq!(run_b(&vec![3, 3, 4, -2, -4]), 10);
        assert_eq!(run_b(&vec![-6, 3, 8, 5, -6]), 5);
        assert_eq!(run_b(&vec![7, 7, -2, -7, -4]), 14);
        assert_eq!(run_b(&vec![1, -2, 3, 1]), 2);
    }
}
