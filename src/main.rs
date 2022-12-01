mod day1;
mod input;

fn main() {
    let filepath =
        "/Users/simeon.aleksandrov/Workspace/Learning/aoc-2022/src/day1/input/dummy-input.txt";
    let data = input::read_file(filepath);
    day1::solution::output(&data);
}
