use solutions;

/// Request enum handles the possible requested solutions by the user.
pub enum Request {
    Day1,
    Day2,
}

impl Request {
    fn solution_runner(self) -> () {
        match self {
            Request::Day1 => solutions::day1::main(),
            Request::Day2 => solutions::day2::main(),
        }
    }

    pub fn run_request(request_arg: &str) -> () {
        let solution_request = match request_arg {
            "1" => Request::Day1,
            "2" => Request::Day2,
            _ => panic!("bad choice"),
        };
        solution_request.solution_runner();
    }
}
