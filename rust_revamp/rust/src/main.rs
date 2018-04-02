use std::env;

pub mod utils;

fn main() {
    let arg = env::args().nth(1).expect("FAILURE : FILE PATH");
    let parsed_args: String = utils::ifile::read_inputs(&arg);
    println!("{:?}", parsed_args);
}
