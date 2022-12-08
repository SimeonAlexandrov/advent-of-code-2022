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
    println!("Max score: {}", max_score);
}

fn calculate_tree_score(forest: &Vec<Vec<u32>>, cell_y: usize, cell_x: usize) -> i32 {
    let lower_limit = forest.len() - 1;
    let right_limit = forest[0].len() - 1;
    if cell_y == 0 || cell_x == 0 || cell_y == lower_limit || cell_x == right_limit {
        return 0;
    }
    let cell = forest[cell_y][cell_x];
    println!("Cell: {}\ty:{}\tx:{}", cell, cell_y, cell_x);

    let column = forest
        .iter()
        .map(|row| row.iter().nth(cell_x).unwrap())
        .collect::<Vec<&u32>>();
    // println!("{:#?}", column);
    let above_count = count_above(cell, &column, cell_y);
    let below_cnt = count_below(cell, &column, cell_y, lower_limit);

    let row = forest.iter().nth(cell_y).unwrap();
    let right_cnt = count_right(cell, &row, cell_x, right_limit);
    let left_cnt = count_left(cell, &row, cell_x);

    return above_count * below_cnt * right_cnt * left_cnt;
}

fn count_above(cell: u32, column: &Vec<&u32>, cell_y: usize) -> i32 {
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
            if *j_cell > &cell && above_cnt == 0 {
                above_cnt = 1;
            }
        }
    }
    return above_cnt;
    // println!("Above cnt: {}", above_cnt);
}

fn count_below(cell: u32, column: &Vec<&u32>, cell_y: usize, below_limit: usize) -> i32 {
    let mut below_cnt = 0;

    let mut below_from_cell: Vec<&u32> = Vec::new();

    for i in (cell_y + 1)..=below_limit {
        below_from_cell.push(column[i]);
    }
    // println!("below: {:#?}", below_from_cell);

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
            if *j_cell > &cell && below_cnt == 0 {
                below_cnt = 1;
            }
        }
    }
    // println!("below cnt: {}", below_cnt);
    return below_cnt;
}

fn count_right(cell: u32, row: &&Vec<u32>, cell_x: usize, right_limit: usize) -> i32 {
    let mut right_cnt = 0;

    let mut right_from_cell: Vec<&u32> = Vec::new();

    for i in (cell_x + 1)..=right_limit {
        right_from_cell.push(&row[i]);
    }
    println!("right: {:#?}", right_from_cell);

    let mut right_break_condition = false;
    for (_, j_cell) in right_from_cell.iter().enumerate() {
        if right_break_condition {
            break;
        }
        if *j_cell <= &cell {
            right_cnt += 1;
        }
        if *j_cell >= &cell {
            right_break_condition = true;
            if *j_cell > &cell && right_cnt == 0 {
                right_cnt = 1;
            }
        }
    }
    println!("right cnt: {}", right_cnt);
    return right_cnt;
}

fn count_left(cell: u32, row: &&Vec<u32>, cell_x: usize) -> i32 {
    let mut left_cnt = 0;

    let mut left_from_cell: Vec<&u32> = Vec::new();

    for i in (0..cell_x).rev() {
        left_from_cell.push(&row[i]);
    }
    // println!("left in reverse: {:#?}", left_from_cell);

    let mut left_break_condition = false;
    for (_, j_cell) in left_from_cell.iter().enumerate() {
        if left_break_condition {
            break;
        }
        if *j_cell <= &cell {
            left_cnt += 1;
        }
        if *j_cell >= &cell {
            left_break_condition = true;
            if *j_cell > &cell && left_cnt == 0 {
                left_cnt = 1;
            }
        }
    }
    // println!("left cnt: {}", left_cnt);
    return left_cnt;
}
