use std::env;
use std::f32;

fn main() {
    let input_int: f32 = read_cmd_line_arg();
    println!("{:?}", &input_int.sqrt().ceil());
}

fn read_cmd_line_arg() -> f32 {
    match env::args().nth(1) {
        Some(a) => a.parse::<f32>().unwrap(),
        None => panic!("FAILURE : FILE PATH"),
    }
}
