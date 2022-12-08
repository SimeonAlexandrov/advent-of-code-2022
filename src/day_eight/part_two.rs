pub fn output(input: &String) {
    println!("{}", input);

    let forest: Vec<Vec<u32>> = input
        .split('\n')
        .map(|line| {
            let row: Vec<u32> = line.chars().map(|ch| ch.to_digit(10).unwrap()).collect();
            row
        })
        .collect();
}
