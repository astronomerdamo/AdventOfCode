use solutions;

/// Request enum handles the possible requested solutions by the user.
pub enum Request {
    Year2017Day1,
    Year2017Day2,
    Year2017Day5,
    Year2017Day6,
    Year2017Day8,
}

impl Request {
    fn solution_runner(self) -> () {
        match self {
            Request::Year2017Day1 => solutions::year2017day1::main(),
            Request::Year2017Day2 => solutions::year2017day2::main(),
            Request::Year2017Day5 => solutions::year2017day5::main(),
            Request::Year2017Day6 => solutions::year2017day6::main(),
            Request::Year2017Day8 => solutions::year2017day8::main(),
        }
    }

    pub fn run_request(request_arg: &str) -> () {
        let solution_request = match request_arg {
            "2017-1" => Request::Year2017Day1,
            "2017-2" => Request::Year2017Day2,
            "2017-5" => Request::Year2017Day5,
            "2017-6" => Request::Year2017Day6,
            "2017-8" => Request::Year2017Day8,
            _ => panic!("bad choice"),
        };
        solution_request.solution_runner();
    }
}
