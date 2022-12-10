pub fn output(input: &String) {
    let input_iter = input.lines();

    let mut cycle_count = 1;
    let mut x = 1;
    let mut total_sum = 0;
    for line in input_iter {
        println!("Just before cycle: {} \t x={}", cycle_count, x);

        let operation = line.split(" ").collect::<Vec<&str>>();

        match operation.len() {
            1 => {
                cycle_count += 1;
                total_sum = update_total_sum(cycle_count, x, total_sum);
            }
            2 => {
                cycle_count += 1;
                total_sum = update_total_sum(cycle_count, x, total_sum);
                let to_add = operation[1].parse::<i32>().unwrap();
                x = x + to_add;
                cycle_count += 1;
                total_sum = update_total_sum(cycle_count, x, total_sum);
            }
            _ => panic!("Unknown command!"),
        };
    }

    println!("total_sum: {}", total_sum);
}

fn update_total_sum(cycle_count: i32, x: i32, total_sum: i32) -> i32 {
    match cycle_count {
        20 => total_sum + 20 * x,
        60 => total_sum + 60 * x,
        100 => total_sum + 100 * x,
        140 => total_sum + 140 * x,
        180 => total_sum + 180 * x,
        220 => total_sum + 220 * x,
        _ => total_sum,
    }
}
