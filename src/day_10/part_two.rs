pub fn output(input: &String) {
    let input_iter = input.lines();

    let mut cycle_count = 0;
    let mut symbols: Vec<char> = vec![];
    let mut sprite_beginning = 1;
    let mut sprite_end = 3;
    for line in input_iter {
        let operation = line.split(" ").collect::<Vec<&str>>();

        match operation.len() {
            1 => {
                cycle_count += 1;
                symbols.push(map_cycle_and_sprite_to_symbol(
                    cycle_count,
                    sprite_beginning,
                    sprite_end,
                ));
                println!(
                    "Cycle: {}\tBeg: {}\tEnd: {}\tSymbols: {:?}",
                    cycle_count, sprite_beginning, sprite_end, &symbols
                );
            }
            2 => {
                cycle_count += 1;
                symbols.push(map_cycle_and_sprite_to_symbol(
                    cycle_count,
                    sprite_beginning,
                    sprite_end,
                ));
                println!(
                    "Cycle: {}\tBeg: {}\tEnd: {}\tSymbols: {:?}",
                    cycle_count, sprite_beginning, sprite_end, &symbols
                );

                cycle_count += 1;

                symbols.push(map_cycle_and_sprite_to_symbol(
                    cycle_count,
                    sprite_beginning,
                    sprite_end,
                ));
                println!(
                    "Cycle: {}\tBeg: {}\tEnd: {}\tSymbols: {:?}",
                    cycle_count, sprite_beginning, sprite_end, &symbols
                );

                let to_add = operation[1].parse::<i32>().unwrap();
                sprite_beginning = sprite_beginning + to_add;
                sprite_end = sprite_end + to_add;
            }
            _ => panic!("Unknown command!"),
        };
    }

    println!("symbols length: {}", &symbols.len());

    for i in 0..6 {
        for j in 0..40 {
            print!("{}", symbols.get(i * 40 + j).unwrap());
        }
        println!();
    }
}

fn map_cycle_and_sprite_to_symbol(cycle_count: i32, beginning: i32, end: i32) -> char {
    if beginning <= cycle_count % 40 && cycle_count % 40 <= end {
        return '#';
    }
    '.'
}
