use std::env;

mod input;
// mod day_one;
// mod day_two;
// mod day_three;
// mod day_four;

mod day_five;

enum Environment {
    Dummy,
    Submission,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut env: Environment = match args.len() {
        1 => Environment::Submission,
        2 => match args[1].as_str() {
            "dummy" => Environment::Dummy,
            "submission" => Environment::Submission,
            _ => panic!("Unexpected argument!"),
        },
        _ => panic!("Unexpected cmd arguments!"),
    };

    let filepath = match env {
        Environment::Dummy => "/Users/simeon.aleksandrov/Workspace/Learning/aoc-2022/src/day_five/input/dummy-input.txt",
        Environment::Submission => "/Users/simeon.aleksandrov/Workspace/Learning/aoc-2022/src/day_five/input/input.txt"
    };
    let data = input::read_file(filepath);

    // day_one::part_one::output(&data);
    // day_one::part_two::output(&data);

    // day_two::part_one::output(&data);
    // day_two::part_two::output(&data);

    // day_three::part_one::output(&data);
    // day_three::part_two::output(&data);

    // day_four::part_one::output(&data);
    // day_four::part_two::output(&data);

    day_five::part_one::output(&data);
}
