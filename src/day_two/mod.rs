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
