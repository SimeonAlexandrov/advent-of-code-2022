use std::env;
use std::error::Error;

mod input;
// mod day_one;
// mod day_two;
// mod day_three;
// mod day_four;
// mod day_five;
// mod day_six;
// mod day_seven;
// mod day_eight;
// mod day_nine;
mod day_10;
mod day_11;

pub enum Environment {
    Dummy,
    Submission,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let env: Environment = match args.len() {
        1 => Environment::Submission,
        2 => match args[1].as_str() {
            "dummy" => Environment::Dummy,
            "submission" => Environment::Submission,
            _ => panic!("Unexpected argument!"),
        },
        _ => panic!("Unexpected cmd arguments!"),
    };

    let filepath = match env {
        Environment::Dummy => {
            "/Users/simeon.aleksandrov/Workspace/Learning/aoc-2022/src/day_11/input/dummy-input.txt"
        }
        Environment::Submission => {
            "/Users/simeon.aleksandrov/Workspace/Learning/aoc-2022/src/day_11/input/input.txt"
        }
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

    // day_five::part_one::output(&data, &env);
    // day_five::part_two::output(&data, &env);

    // day_six::part_one_and_two::output(&data, 4);
    // day_six::part_one_and_two::output(&data, 14);

    // day_seven::part_one::output(&data);
    // day_eight::part_one::output(&data);
    // day_eight::part_two::output(&data);

    // day_nine::part_one::output(&data);

    // day_10::part_one::output(&data);
    // day_10::part_two::output(&data);

    day_11::part_one::output(&data);
    Ok(())
}
