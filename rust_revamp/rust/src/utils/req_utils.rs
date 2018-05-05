use solutions;

/// Request enum handles the possible requested solutions by the user.
pub enum Request {
    Day1,
    Day2,
    Day5,
    Day8,
}

impl Request {
    fn solution_runner(self) -> () {
        match self {
            Request::Day1 => solutions::day1::main(),
            Request::Day2 => solutions::day2::main(),
            Request::Day5 => solutions::day5::main(),
            Request::Day8 => solutions::day8::main(),
        }
    }

    pub fn run_request(request_arg: &str) -> () {
        let solution_request = match request_arg {
            "1" => Request::Day1,
            "2" => Request::Day2,
            "5" => Request::Day5,
            "8" => Request::Day8,
            _ => panic!("bad choice"),
        };
        solution_request.solution_runner();
    }
}
