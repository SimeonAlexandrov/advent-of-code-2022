mod input;
// mod day_one;
// mod day_two;
// mod day_three;

mod day_four;

fn main() {
    let _filepath_dummy =
        "/Users/simeon.aleksandrov/Workspace/Learning/aoc-2022/src/day_four/input/dummy-input.txt";
    let _filepath =
        "/Users/simeon.aleksandrov/Workspace/Learning/aoc-2022/src/day_four/input/input.txt";
    let data = input::read_file(_filepath_dummy);

    // day_one::part_one::output(&data);
    // day_one::part_two::output(&data);

    // day_two::part_one::output(&data);
    // day_two::part_two::output(&data);

    // day_three::part_one::output(&data);
    // day_three::part_two::output(&data);

    day_four::part_one::output(&data);
}
