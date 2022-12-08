pub fn output(input: &String) {
    println!("{}", input);

    let forest: Vec<Vec<u32>> = input
        .split('\n')
        .map(|line| {
            let row: Vec<u32> = line.chars().map(|ch| ch.to_digit(10).unwrap()).collect();
            row
        })
        .collect();

    let mut max_score = 0;
    for (i, row) in forest.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let tree_score = calculate_tree_score(&forest, i, j);
            if tree_score > max_score {
                max_score = tree_score;
            }
        }
    }
}

fn calculate_tree_score(forest: &Vec<Vec<u32>>, cell_y: usize, cell_x: usize) -> u32 {
    let lower_limit = forest.len() - 1;
    let right_limit = forest[0].len() - 1;
    if cell_y == 0 || cell_x == 0 || cell_y == lower_limit || cell_x == right_limit {
        return 0;
    }
    let mut initial_multiplier = 1;
    let cell = forest[cell_y][cell_x];
    println!("Cell: {}\ty:{}\tx:{}", cell, cell_y, cell_x);

    let column = forest
        .iter()
        .map(|row| row.iter().nth(cell_x).unwrap())
        .collect::<Vec<&u32>>();
    // println!("{:#?}", column);
    let above_count = count_above(cell, &column, cell_y);
    let below_cnt = count_below(cell, &column, cell_y, lower_limit);
    0
}

fn count_above(cell: u32, column: &Vec<&u32>, cell_y: usize) {
    let mut above_cnt = 0;

    let mut above_from_cell: Vec<&u32> = Vec::new();

    for i in (0..cell_y).rev() {
        above_from_cell.push(column[i]);
    }
    // println!("Above in reverse: {:#?}", above_from_cell);

    let mut above_break_condition = false;
    for (_, j_cell) in above_from_cell.iter().enumerate() {
        if above_break_condition {
            break;
        }
        if *j_cell <= &cell {
            above_cnt += 1;
        }
        if *j_cell >= &cell {
            above_break_condition = true;
        }
    }
    // println!("Above cnt: {}", above_cnt);
}

fn count_below(cell: u32, column: &Vec<&u32>, cell_y: usize, below_limit: usize) {
    let mut below_cnt = 0;

    let mut below_from_cell: Vec<&u32> = Vec::new();

    for i in (cell_y + 1)..=below_limit {
        below_from_cell.push(column[i]);
    }
    println!("below in reverse: {:#?}", below_from_cell);

    let mut below_break_condition = false;
    for (_, j_cell) in below_from_cell.iter().enumerate() {
        if below_break_condition {
            break;
        }
        if *j_cell <= &cell {
            below_cnt += 1;
        }
        if *j_cell >= &cell {
            below_break_condition = true;
        }
    }
    println!("below cnt: {}", below_cnt);
}
