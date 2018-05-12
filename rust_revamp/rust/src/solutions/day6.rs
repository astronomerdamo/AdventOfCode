use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::Path;
use utils;

fn load_raw_input() -> String {
    let path = Path::new("./inputs/day6.txt");
    println!("Loading inputs file: {:?}", &path);
    utils::file_utils::read_inputs(path)
}

fn parse_input() -> HashMap<usize, usize> {
    load_raw_input()
        .split_whitespace()
        .enumerate()
        .map(|(i, x)| (i, x.trim().parse::<usize>().unwrap()))
        .collect()
}

fn find_max_memory_bank(memory: &HashMap<usize, usize>) -> (usize, usize) {
    memory.iter().fold((0, 0), |mut acc, s| {
        if (acc.1 == *s.1 && acc.0 > *s.0) || acc.1 < *s.1 {
            acc = (*s.0, *s.1);
        }
        acc
    })
}

fn reallocate_inner(memory: &mut HashMap<usize, usize>, cycling_index: usize, cycle_count: usize) {
    let mem_len = memory.len();
    if memory.contains_key(&(cycling_index - (cycle_count * mem_len))) {
        if let Some(x) = memory.get_mut(&(cycling_index - (cycle_count * mem_len))) {
            *x += 1;
        }
    } else {
        reallocate_inner(memory, cycling_index, cycle_count + 1)
    }
}

fn reallocate_memory_bank(mut memory: &mut HashMap<usize, usize>) {
    let max_memory_bank = find_max_memory_bank(&memory);

    if let Some(x) = memory.get_mut(&max_memory_bank.0) {
        *x = 0
    }

    for i in 1..max_memory_bank.1 + 1 {
        reallocate_inner(&mut memory, max_memory_bank.0 + i, 0);
    }
}

fn hash_memory<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn redistribution_inner(
    mut memory: &mut HashMap<usize, usize>,
    mut state_history: &mut HashMap<u64, usize>,
    mut iter_count: &mut usize,
) -> (usize, usize) {
    match state_history.insert(
        hash_memory(&memory.iter().collect::<Vec<(&usize, &usize)>>()),
        *iter_count,
    ) {
        Some(k) => (*iter_count, *iter_count - k),
        None => {
            *iter_count += 1;
            reallocate_memory_bank(&mut memory);
            redistribution_inner(&mut memory, &mut state_history, &mut iter_count)
        }
    }
}

fn run_redistributions(input: &HashMap<usize, usize>) -> (usize, usize) {
    let mut memory = input.clone();
    let mut state_history: HashMap<u64, usize> = HashMap::new();
    let mut iter_count: usize = 0;
    redistribution_inner(&mut memory, &mut state_history, &mut iter_count)
}

fn run_a(redistributions: &(usize, usize)) -> usize {
    redistributions.0
}

fn run_b(redistributions: &(usize, usize)) -> usize {
    redistributions.1
}

pub fn main() {
    let input = parse_input();
    let redistributions = run_redistributions(&input);
    println!("Day 6a: {:?}", &run_a(&redistributions));
    println!("Day 6b: {:?}", &run_b(&redistributions));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> HashMap<usize, usize> {
        let mut test_input_map = HashMap::new();
        for &(k, v) in vec![(0, 0), (1, 2), (2, 7), (3, 0)].iter() {
            test_input_map.insert(k, v);
        }
        test_input_map
    }

    #[test]
    fn test_run_a() {
        assert_eq!(run_a(&run_redistributions(&test_input())), 5);
    }

    #[test]
    fn test_run_b() {
        assert_eq!(run_b(&run_redistributions(&test_input())), 4);
    }
}
