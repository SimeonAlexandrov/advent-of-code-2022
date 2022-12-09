use regex::Regex;

use crate::day_nine::table::Table;

pub fn output(input: &String) {
    println!("{input}");

    let commands = input.split('\n');

    let table_dimensions = determine_table_size(&input);

    println!("{:#?}", table_dimensions);

    let mut table = Table::new(table_dimensions.0, table_dimensions.1);

    table.print();
}

fn determine_table_size(input: &String) -> (usize, usize) {
    let mut right_cnt: usize = 0;
    let mut left_cnt: usize = 0;
    let mut down_cnt: usize = 0;
    let mut up_cnt: usize = 0;
    let command_regex = Regex::new("^([A-Z]) ([0-9]*)").unwrap();
    let _ = input.split('\n').for_each(|line| {
        let caps = command_regex.captures(line).unwrap();
        let command_payload = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let _ = match caps.get(1).unwrap().as_str() {
            "R" => {
                right_cnt += command_payload;
            }
            "L" => {
                left_cnt += command_payload;
            }
            "D" => {
                down_cnt += command_payload;
            }
            "U" => {
                up_cnt += command_payload;
            }
            _ => panic!("Unexpected command!"),
        };
    });
    // println!("Right cnt: {right_cnt}");
    // println!("Left cnt: {left_cnt}");
    // println!("down cnt: {down_cnt}");
    // println!("up cnt: {up_cnt}");

    // We want to start in the center of the table so *2
    ((up_cnt + down_cnt) * 2, (left_cnt + right_cnt) * 2)
}
