use super::is_visible::is_visible;
pub fn output(input: &String) {
    let forest: Vec<Vec<u32>> = input
        .split('\n')
        .map(|line| {
            let row: Vec<u32> = line.chars().map(|ch| ch.to_digit(10).unwrap()).collect();
            row
        })
        .collect();

    let mut visible_count: u32 = 0;

    for (i, row) in forest.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if is_visible(&forest, i, j) {
                visible_count += 1;
            } else {
                // println!("Not visible cell: {} \n\ty={},\tx={}", cell, i, j);
                if i == 52 && j == 3 {
                    println!("Not visible cell: {} \n\ty={},\tx={}", cell, i, j);
                }
            }
        }
    }

    println!("Result {}", visible_count);
}
