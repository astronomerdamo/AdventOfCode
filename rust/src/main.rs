use std::env;

pub mod utils;
pub mod solutions;

fn main() {
    let request_arg: String = env::args().nth(1).expect("FAILURE: DAY PICK");
    utils::req_utils::Request::run_request(request_arg.as_str());
}
