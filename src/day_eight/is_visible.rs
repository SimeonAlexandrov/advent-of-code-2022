pub fn is_visible(forest: &Vec<Vec<u32>>, cell_y: usize, cell_x: usize) -> bool {
    return is_visible_top_bottom(&forest, cell_y, cell_x);
    // || is_visible_left_right(&forest, cell_y, cell_x);
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
                return false;
            }
            true
        }
    }
}

fn is_visible_left_right(forest: &Vec<Vec<u32>>, cell_x: usize, cell_y: usize) -> bool {
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

            let row = forest.iter().nth(cell_y).unwrap();

            println!("Row: {:#?}", row);

            for (i, el) in row.iter().enumerate() {
                if i < cell_x && i32::try_from(*el).ok().unwrap() > left_max {
                    left_max = i32::try_from(*el).ok().unwrap();
                }

                if i > cell_x && i32::try_from(*el).ok().unwrap() > right_max {
                    right_max = i32::try_from(*el).ok().unwrap();
                }
            }

            println!("left_max: {}\tright_max={}", left_max, right_max);
            if left_max >= forest[cell_y][cell_x].try_into().unwrap()
                && right_max >= forest[cell_y][cell_x].try_into().unwrap()
            {
                return false;
            }
            true
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
