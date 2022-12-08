pub fn output(input: &String) {
    let forest: Vec<Vec<u32>> = input
        .split('\n')
        .map(|line| {
            let row: Vec<u32> = line.chars().map(|ch| ch.to_digit(10).unwrap()).collect();
            row
        })
        .collect();
}

fn is_visible(forest: &Vec<Vec<u32>>, cell_x: usize, cell_y: usize) {}
