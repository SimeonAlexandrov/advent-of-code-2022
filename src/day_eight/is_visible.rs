pub fn is_visible(forest: &Vec<Vec<u32>>, cell_y: usize, cell_x: usize) -> bool {
    let top_bottom = is_visible_top_bottom(&forest, cell_y, cell_x);
    let left_right = is_visible_left_right(&forest, cell_y, cell_x);
    if cell_y == 52 && cell_x == 3 {
        println!("top_bottom: {}\t left_right: {}", top_bottom, left_right);
    }
    return top_bottom || left_right;
}

fn is_visible_top_bottom(forest: &Vec<Vec<u32>>, cell_y: usize, cell_x: usize) -> bool {
    let lower_limit = forest.len() - 1;
    let right_limit = forest[0].len() - 1;
    match (cell_y, cell_x) {
        (0, _) => true,
        (_, 0) => true,
        (y, x) => {
            if x == lower_limit || y == lower_limit || x == right_limit || y == right_limit {
                return true;
            }
            let mut above_max: i32 = -1;
            let mut below_max: i32 = -1;

            let column = forest
                .iter()
                .map(|row| row.iter().nth(cell_x).unwrap())
                .collect::<Vec<&u32>>();

            // println!("Column: {:#?}", column);

            for (i, el) in column.iter().enumerate() {
                if i < cell_y && i32::try_from(**el).ok().unwrap() > above_max {
                    above_max = i32::try_from(**el).ok().unwrap();
                }

                if i > cell_y && i32::try_from(**el).ok().unwrap() > below_max {
                    below_max = i32::try_from(**el).ok().unwrap();
                }
            }

            // println!("above_max: {}\tbelow_max={}", above_max, below_max);
            if above_max >= forest[cell_y][cell_x].try_into().unwrap()
                && below_max >= forest[cell_y][cell_x].try_into().unwrap()
            {
                if cell_y == 52 && cell_x == 3 {
                    println!("TB NOT Visible!");
                }
                return false;
            }
            if cell_y == 52 && cell_x == 3 {
                println!("TB Visible!");
            }
            true
        }
    }
}

fn is_visible_left_right(forest: &Vec<Vec<u32>>, cell_y: usize, cell_x: usize) -> bool {
    let lower_limit = forest.len() - 1;
    let right_limit = forest[0].len() - 1;
    match (cell_y, cell_x) {
        (0, _) => true,
        (_, 0) => true,
        (y, x) => {
            if x == lower_limit || y == lower_limit || x == right_limit || y == right_limit {
                return true;
            }
            let mut left_max = 0;
            let mut right_max = 0;

            if cell_y == 52 && cell_x == 3 {
                println!("cell: {}", forest[cell_y][cell_x]);
            }

            let row = forest.iter().nth(cell_y).unwrap();

            // println!("Row: {:#?}", row);

            for (i, el) in row.iter().enumerate() {
                if i < cell_x && i32::try_from(*el).ok().unwrap() > left_max {
                    if cell_y == 52 && cell_x == 3 {
                        println!("Left max: {}", left_max);
                        println!("Element left of 5: {}", i32::try_from(*el).ok().unwrap());
                    }
                    left_max = i32::try_from(*el).ok().unwrap();
                }

                if i > cell_x && i32::try_from(*el).ok().unwrap() > right_max {
                    if cell_y == 52 && cell_x == 3 {
                        // println!("Right max: {}", right_max);
                        // println!("Element right of 5: {}", i32::try_from(*el).ok().unwrap());
                    }
                    right_max = i32::try_from(*el).ok().unwrap();
                }
            }
            if cell_y == 52 && cell_x == 3 {
                let tmp: i32 = forest[cell_y][cell_x].try_into().unwrap();
                println!("left_max: {}\t{}\tright_max={}", left_max, tmp, right_max);
                println!(
                    "left_max: {}\t>\ttmp={}\t{}",
                    left_max,
                    tmp,
                    left_max >= tmp
                );
            }
            let is_not_visible_from_the_left =
                left_max >= forest[cell_y][cell_x].try_into().unwrap();
            let is_not_visible_from_the_right =
                right_max >= forest[cell_y][cell_x].try_into().unwrap();
            if is_not_visible_from_the_left && is_not_visible_from_the_right {
                if cell_y == 52 && cell_x == 3 {
                    println!("LR Not Visible!");
                }
                return false;
            }

            if cell_y == 52 && cell_x == 3 {
                println!("LR Visible!");
            }
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Setup {
        forest: Vec<Vec<u32>>,
    }
    impl Setup {
        fn new() -> Box<Self> {
            Box::new(Self {
                forest: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
            })
        }
    }
    mod top {
        use super::*;
        #[test]
        fn test_is_visible_top_corner() {
            // Given
            let setup = *Setup::new();
            // When
            let is_visible = is_visible_top_bottom(&setup.forest, 0, 0);
            // Then
            assert_eq!(is_visible, true);
        }

        #[test]
        fn test_is_visible_top_inner() {
            // Given
            let setup = *Setup::new();
            // When
            let is_visible = is_visible_top_bottom(&setup.forest, 1, 1);
            // Then
            assert_eq!(is_visible, true);
        }

        #[test]
        fn test_is_visible_top_side() {
            // Given
            let setup = *Setup::new();
            // When
            let is_visible = is_visible_top_bottom(&setup.forest, 0, 1);
            // Then
            assert_eq!(is_visible, true);
        }

        #[test]
        fn test_not_visible_top() {
            // Given
            let forest = vec![vec![0, 6, 0], vec![0, 5, 0], vec![0, 0, 0]];
            // When
            let is_visible = is_visible_top_bottom(&forest, 1, 1);
            // Then
            assert_eq!(is_visible, true);
        }

        #[test]
        fn test_not_visible_top_equal() {
            // Given
            let forest = vec![vec![0, 5, 0], vec![0, 5, 0], vec![0, 0, 0]];
            // When
            let is_visible = is_visible_top_bottom(&forest, 1, 1);
            // Then
            assert_eq!(is_visible, true);
        }

        #[test]
        fn test_is_visible_top_not_adjacent() {
            // Given
            let forest = vec![
                vec![0, 5, 0, 0],
                vec![0, 0, 1, 0],
                vec![7, 4, 2, 7],
                vec![0, 6, 0, 0],
            ];
            // When
            // println!("{}", forest[2][1]);
            let is_visible = is_visible_top_bottom(&forest, 2, 1);
            // Then
            assert_eq!(is_visible, false);
        }

        #[test]
        fn test_is_visible_botoom_not_adjacent() {
            // Given
            let forest = vec![
                vec![0, 5, 0, 0],
                vec![0, 4, 1, 0],
                vec![7, 0, 2, 7],
                vec![0, 6, 0, 0],
            ];
            // When
            // println!("{}", forest[2][1]);
            let is_visible = is_visible_top_bottom(&forest, 1, 1);
            // Then
            assert_eq!(is_visible, false);
        }

        #[test]
        fn test_is_visible_left_right_not_adjacent() {
            // Given
            let forest = vec![
                vec![0, 5, 0, 0],
                vec![0, 4, 1, 0],
                vec![7, 0, 2, 7],
                vec![0, 6, 0, 0],
            ];
            // When
            // println!("{}", forest[2][2]);
            let is_visible = is_visible_left_right(&forest, 2, 2);
            // Then
            assert_eq!(is_visible, false);
        }
    }

    mod is_visible {
        use super::*;
        #[test]
        fn test_is_visible_0() {
            // Given
            let setup = *Setup::new();
            // When
            let is_visible = is_visible(&setup.forest, 0, 0);
            // Then
            assert_eq!(is_visible, true);
        }

        #[test]
        fn test_is_visible_1() {
            // Given
            let forest = vec![vec![0, 0, 0], vec![0, 5, 0], vec![0, 6, 0]];
            // When
            let is_visible = is_visible(&forest, 1, 1);
            // Then
            assert_eq!(is_visible, true);
        }

        #[test]
        fn test_is_visible_2() {
            // Given
            let forest = vec![vec![0, 6, 0], vec![7, 5, 5], vec![0, 6, 0]];
            // When
            let is_visible = is_visible(&forest, 1, 1);
            // Then
            assert_eq!(is_visible, false);
        }

        #[test]
        fn test_is_visible_not_adjacent() {
            // Given
            let forest = vec![
                vec![0, 5, 0, 0],
                vec![0, 0, 0, 0],
                vec![7, 4, 0, 7],
                vec![0, 6, 0, 0],
            ];
            // When
            let is_visible = is_visible(&forest, 2, 1);
            // Then
            assert_eq!(is_visible, false);
        }

        #[test]
        fn test_is_visible_from_input() {
            // Given
            let forest = vec![
                vec![2, 3, 2, 2],
                vec![0, 0, 1, 2],
                vec![2, 0, 0, 0],
                vec![0, 1, 2, 0],
            ];
            // When
            let is_visible = is_visible(&forest, 2, 2);
            // Then
            assert_eq!(is_visible, false);
        }

        #[test]
        fn test_is_visible_from_input_left_right() {
            // Given
            let forest = vec![
                vec![2, 3, 2, 2],
                vec![0, 0, 1, 2],
                vec![2, 0, 0, 0],
                vec![0, 1, 2, 0],
            ];
            // When
            let is_visible = is_visible_left_right(&forest, 2, 2);
            // Then
            assert_eq!(is_visible, false);
        }
    }
}
