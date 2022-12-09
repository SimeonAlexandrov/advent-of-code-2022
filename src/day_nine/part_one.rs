use regex::Regex;

pub fn output(input: &String) {
    println!("{input}");

    let commands = input.split('\n');

    let table_dimensions = determine_table_size(&input);

    println!("{:#?}", table_dimensions);
}

fn determine_table_size(input: &String) -> (u32, u32) {
    let mut right_cnt = 0;
    let mut left_cnt = 0;
    let mut down_cnt = 0;
    let mut up_cnt = 0;
    let command_regex = Regex::new("^([A-Z]) ([0-9]*)").unwrap();
    let _ = input.split('\n').for_each(|line| {
        let caps = command_regex.captures(line).unwrap();
        let command_payload = caps.get(2).unwrap().as_str().parse::<u32>().unwrap();
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
    println!("Right cnt: {right_cnt}");
    println!("Left cnt: {left_cnt}");
    println!("down cnt: {down_cnt}");
    println!("up cnt: {up_cnt}");

    // We want to start in the center of the table so *2
    ((up_cnt + down_cnt) * 2, (left_cnt + right_cnt) * 2)
}
