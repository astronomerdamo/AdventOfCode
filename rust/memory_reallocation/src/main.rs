use std::fs;
use std::env;
use std::io::Read;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct Bank {
    index: u8,
    blocks: u16,
}

fn find_max_bank_by_index(mem_banks: &Vec<Bank>) -> (u8, u16) {
    let max_bank = mem_banks.iter().cloned().fold(
        Bank {
            index: 0,
            blocks: 0,
        },
        |mut acc, s| {
            if (acc.blocks == s.blocks && acc.index > s.index) || acc.blocks < s.blocks {
                acc = s;
            }
            acc
        },
    );
    (max_bank.index, max_bank.blocks)
}

fn reset_max_bank_blocks(mem_banks: &mut Vec<Bank>, max_bank_index: &u8) {
    for bank in mem_banks.iter_mut() {
        if bank.index == *max_bank_index {
            (*bank).blocks = 0;
        }
    }
}

fn reallocate_memory_banks(mut mem_banks: &mut Vec<Bank>) {
    let (mut max_bank_index, max_bank_blocks) = find_max_bank_by_index(&mem_banks);
    let mut bank_index = max_bank_index.clone();
    reset_max_bank_blocks(&mut mem_banks, &mut max_bank_index);

    for _ in 0..max_bank_blocks {
        if (bank_index + 1) >= (mem_banks.len() as u8) {
            bank_index = 0;
        } else {
            bank_index += 1;
        }

        for bank in mem_banks.iter_mut() {
            if bank.index == bank_index {
                (*bank).blocks += 1;
            }
        }
    }
}

fn main() {
    let mut memory = parse_memory_banks();
    let mut memory_states: Vec<(u32, Vec<Bank>)> = Vec::new();
    let mut debug_count = 0u32;

    let (loop_count, loop_incidence) = loop {
        reallocate_memory_banks(&mut memory);

        debug_count += 1;

        let matched_memory_state = match memory_states
            .iter()
            .cloned()
            .filter(|x| x.1 == memory)
            .next()
        {
            Some(state) => (true, state.0),
            None => (false, 0u32),
        };

        if matched_memory_state.0 {
            break (debug_count, matched_memory_state.1);
        };

        memory_states.push((debug_count, memory.clone()));
    };

    println!("Number of memory bank moves {:?}", &loop_count);
    println!(
        "Reallocation moves between loop boundries {:?}",
        &loop_count - &loop_incidence
    );
}

fn parse_memory_banks() -> Vec<Bank> {
    let mut index = 0u8;
    open_file_read_contents()
        .split_whitespace()
        .map(|x| {
            let cl_bank = Bank {
                index: index,
                blocks: x.parse::<u16>().unwrap(),
            };
            index += 1;
            cl_bank
        })
        .collect()
}

fn open_file_read_contents() -> String {
    let path = match env::args().nth(1) {
        Some(p) => p,
        None => panic!("FAILURE : FILE PATH"),
    };

    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => panic!("FAILURE : OPEN FILE"),
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(s) => s,
        Err(_) => panic!("FAILURE : READ FILE"),
    };
    buffer
}
