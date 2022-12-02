#[allow(dead_code)]
pub mod part_one {
    #[derive(Debug, Clone)]
    enum Shape {
        Rock,
        Paper,
        Scissors,
    }
    #[derive(Debug)]
    struct Hand {
        shape: Shape,
        points: u8,
    }

    #[derive(Debug)]
    struct Battle {
        opponent: Hand,
        me: Hand,
    }

    impl Battle {
        fn resolve_battle(&self) -> u8 {
            let battle_score = match (&self.opponent.shape, &self.me.shape) {
                (Shape::Rock, Shape::Scissors) => 0,
                (Shape::Paper, Shape::Rock) => 0,
                (Shape::Scissors, Shape::Paper) => 0,
                (Shape::Rock, Shape::Rock) => 3,
                (Shape::Paper, Shape::Paper) => 3,
                (Shape::Scissors, Shape::Scissors) => 3,
                (Shape::Rock, Shape::Paper) => 6,
                (Shape::Paper, Shape::Scissors) => 6,
                (Shape::Scissors, Shape::Rock) => 6,
            };

            battle_score + self.me.points
        }
    }

    fn match_char_to_shape(letter: &char) -> Shape {
        match letter {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            'X' => Shape::Rock,
            'Y' => Shape::Paper,
            'Z' => Shape::Scissors,
            _ => panic!("Unexpected shape in input!"),
        }
    }

    fn match_shape_to_points(shape: Shape) -> u8 {
        match shape {
            Shape::Rock => 1,     // Rock
            Shape::Paper => 2,    // Paper
            Shape::Scissors => 3, // Scissors
        }
    }

    fn match_line_to_battle(line: &str) -> Battle {
        match line {
            _ => {
                let mut lines_with_shapes = line
                    .split(' ')
                    .map(|s| s.chars().next().unwrap_or_default())
                    .map(|ch| match_char_to_shape(&ch));

                let opponent_shape = lines_with_shapes.next().unwrap();
                let my_shape = lines_with_shapes.next().unwrap();
                Battle {
                    opponent: Hand {
                        shape: opponent_shape,
                        points: 0, // We don't care for opponent points
                    },
                    me: Hand {
                        shape: my_shape.clone(),
                        points: match_shape_to_points(my_shape.clone()),
                    },
                }
            }
        }
    }

    pub fn output(input: &String) {
        let battles: Vec<Battle> = input.split("\n").map(|s| match_line_to_battle(s)).collect();
        println!("{:#?}", battles);
        let scores: Vec<u8> = battles.iter().map(|b| b.resolve_battle()).collect();
        println!("Scores: {:#?}", scores);
        println!(
            "Result: {}",
            scores.iter().map(|&score| score as u32).sum::<u32>()
        )
    }
}

pub mod part_two {
    #[derive(Debug, Clone)]
    enum Shape {
        Rock,
        Paper,
        Scissors,
    }

    #[derive(Debug, Clone)]
    enum ExpectedResult {
        Win,
        Draw,
        Lose,
    }

    impl ExpectedResult {
        fn resolve_points(expected_result: &ExpectedResult) -> u8 {
            match expected_result {
                ExpectedResult::Win => 6,
                ExpectedResult::Draw => 3,
                ExpectedResult::Lose => 0,
            }
        }
    }

    #[derive(Debug)]
    struct Hand {
        shape: Shape,
        points: u8,
    }

    impl Hand {
        fn resolve_points(shape: &Shape) -> u8 {
            match shape {
                Shape::Rock => 1,     // Rock
                Shape::Paper => 2,    // Paper
                Shape::Scissors => 3, // Scissors
            }
        }
    }

    #[derive(Debug)]
    struct Battle {
        opponent: Hand,
        expected_result: ExpectedResult,
        me: Hand,
    }

    fn match_char_to_shape(letter: &char) -> Shape {
        match letter {
            'A' => Shape::Rock,
            'B' => Shape::Paper,
            'C' => Shape::Scissors,
            _ => panic!("Unexpected shape in input!"),
        }
    }

    fn match_char_to_result(letter: &char) -> ExpectedResult {
        match letter {
            'X' => ExpectedResult::Lose,
            'Y' => ExpectedResult::Draw,
            'Z' => ExpectedResult::Win,
            _ => panic!("Unexpected result in input!"),
        }
    }

    fn match_line_to_battle(line: &str) -> Battle {
        match line {
            _ => {
                let mut lines_with_shapes_and_results = line
                    .split(' ')
                    .map(|s| s.chars().next().unwrap_or_default());

                let opponent_shape =
                    match_char_to_shape(&lines_with_shapes_and_results.next().unwrap());
                let expected_result =
                    match_char_to_result(&lines_with_shapes_and_results.next().unwrap());

                return Battle::new(opponent_shape, expected_result);
            }
        }
    }

    impl Battle {
        fn new(opponent_shape: Shape, expected_result: ExpectedResult) -> Self {
            let me_shape = Self::resolve_battle(opponent_shape.clone(), expected_result.clone());
            Self {
                opponent: Hand {
                    shape: opponent_shape,
                    points: 0,
                },
                expected_result: expected_result.clone(),
                me: Hand {
                    shape: me_shape.clone(),
                    points: Hand::resolve_points(&me_shape),
                },
            }
        }
        fn resolve_battle(opponent_shape: Shape, expected_result: ExpectedResult) -> Shape {
            match (opponent_shape, expected_result) {
                (Shape::Rock, ExpectedResult::Win) => Shape::Paper,
                (Shape::Rock, ExpectedResult::Draw) => Shape::Rock,
                (Shape::Rock, ExpectedResult::Lose) => Shape::Scissors,
                (Shape::Paper, ExpectedResult::Win) => Shape::Scissors,
                (Shape::Paper, ExpectedResult::Draw) => Shape::Paper,
                (Shape::Paper, ExpectedResult::Lose) => Shape::Rock,
                (Shape::Scissors, ExpectedResult::Win) => Shape::Rock,
                (Shape::Scissors, ExpectedResult::Draw) => Shape::Scissors,
                (Shape::Scissors, ExpectedResult::Lose) => Shape::Paper,
            }
        }
    }

    pub fn output(input: &String) {
        let battles: Vec<Battle> = input.split("\n").map(|s| match_line_to_battle(s)).collect();
        println!("{:#?}", battles);

        let scores: Vec<u8> = battles
            .iter()
            .map(|b| {
                ExpectedResult::resolve_points(&b.expected_result)
                    + Hand::resolve_points(&b.me.shape)
            })
            .collect();

        println!("Scores: {:#?}", scores);
        println!(
            "Result: {}",
            scores.iter().map(|&score| score as u32).sum::<u32>()
        )
    }
}
