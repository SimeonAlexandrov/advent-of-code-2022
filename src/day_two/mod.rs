pub mod part_one {
    #[derive(Debug)]
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
                (Shape::Scissors, Shape::Rock) => 3,
                _ => panic!(
                    "Unexpected shapes during battle opp:{:#?} me:{:#?}",
                    &self.opponent.shape, &self.me.shape
                ),
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

    fn match_shape_to_points(shape: &char) -> u8 {
        match shape {
            'X' => 1, // Rock
            'Y' => 2, // Paper
            'Z' => 3, // Scissors
            _ => 0,
        }
    }

    fn match_line_to_battle(line: &str) -> Battle {
        match line {
            _ => {
                let hands: Vec<char> = line.split(' ').map(|s| s.chars().next().unwrap()).collect();
                Battle {
                    opponent: Hand {
                        shape: match_char_to_shape(&hands[0]),
                        points: 0, // We don't care for opponent points
                    },
                    me: Hand {
                        shape: match_char_to_shape(&hands[0]),
                        points: match_shape_to_points(&hands[1]),
                    },
                }
            }
        }
    }

    pub fn output(input: &String) {
        let battles: Vec<Battle> = input.split("\n").map(|s| match_line_to_battle(s)).collect();
        println!("{:#?}", battles);
        let scores: Vec<u8> = battles.iter().map(|b| b.resolve_battle()).collect();
        println!(
            "Result: {}",
            scores.iter().map(|&score| score as u32).sum::<u32>()
        )
    }
}
