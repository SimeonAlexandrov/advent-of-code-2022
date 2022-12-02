// mod day_one;
mod day_two;
mod input;

fn main() {
    let filepath =
        "/Users/simeon.aleksandrov/Workspace/Learning/aoc-2022/src/day_two/input/dummy-input.txt";
    let data = input::read_file(filepath);

    // day_one::part_one::output(&data);
    // day_one::part_two::output(&data);

    day_two::part_one::output(&data);
}
