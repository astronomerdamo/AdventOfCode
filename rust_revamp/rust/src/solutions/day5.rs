use std::collections::HashMap;
use std::path::Path;
use utils;

fn load_raw_input() -> String {
    let path = Path::new("./inputs/day5.txt");
    println!("Loading inputs file: {:?}", &path);
    utils::file_utils::read_inputs(path)
}

fn parse_input() -> HashMap<usize, isize> {
    load_raw_input()
        .lines()
        .enumerate()
        .map(|(i, x)| (i, x.trim().parse::<isize>().unwrap()))
        .collect()
}

fn run_instructions(input: &HashMap<usize, isize>, part: char) -> usize {
    let mut instructions = input.clone();
    let mut index: usize = 0;
    let mut steps: usize = 0;
    let part_b: char = 'b';
    while instructions.contains_key(&index) {
        let value = instructions.get_mut(&index).unwrap();

        index = if value.is_positive() {
            index + *value as usize
        } else {
            index - value.abs() as usize
        };

        *value += if *value >= 3 && part == part_b { -1 } else { 1 };
        steps += 1;
    }
    steps
}

fn run_a(input: &HashMap<usize, isize>) -> usize {
    run_instructions(&input, 'a')
}

fn run_b(input: &HashMap<usize, isize>) -> usize {
    run_instructions(&input, 'b')
}

pub fn main() {
    let input: HashMap<usize, isize> = parse_input();
    println!("Day 5a: {:?}", &run_a(&input));
    println!("Day 5b: {:?}", &run_b(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> HashMap<usize, isize> {
        let mut test_input_map = HashMap::new();
        for &(k, v) in vec![(0, 0), (1, 3), (2, 0), (3, 1), (4, -3)].iter() {
            test_input_map.insert(k, v);
        }
        test_input_map
    }

    #[test]
    fn test_run_a() {
        assert_eq!(run_a(&test_input()), 5);
    }

    #[test]
    fn test_run_b() {
        assert_eq!(run_b(&test_input()), 10);
    }
}
