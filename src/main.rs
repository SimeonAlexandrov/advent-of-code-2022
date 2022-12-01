mod day_one;
mod input;

fn main() {
    let filepath =
        "/Users/simeon.aleksandrov/Workspace/Learning/aoc-2022/src/day_one/input/dummy-input.txt";
    let data = input::read_file(filepath);
    day_one::part_one::output(&data);
    day_one::part_two::output(&data);
}
