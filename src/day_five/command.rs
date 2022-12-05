use regex::Regex;

#[derive(Debug)]
pub struct Command {
    pub how_many: usize,
    pub source: usize,
    pub dest: usize,
}

impl Command {
    pub fn new(s: &str) -> Self {
        let re =
            Regex::new(r"^move ([0-9]|[1-9][0-9]) from ([0-9]|[1-9][0-9]) to ([0-9]|[1-9][0-9])")
                .unwrap();
        let caps = re.captures(s).unwrap();
        Self {
            how_many: caps.get(1).unwrap().as_str().parse::<usize>().unwrap(),
            source: caps.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1, // to use indexes
            dest: caps.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1,   // to use indexes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_1() {
        // Given
        let line = "move 1 from 2 to 1";

        // When
        let cmd = Command::new(line);

        // Then
        assert_eq!(cmd.how_many, 1);
        assert_eq!(cmd.source, 1);
        assert_eq!(cmd.dest, 0);
    }

    #[test]
    fn test_new_2() {
        // Given
        let line = "move 3 from 1 to 3";

        // When
        let cmd = Command::new(line);

        // Then
        assert_eq!(cmd.how_many, 3);
        assert_eq!(cmd.source, 0);
        assert_eq!(cmd.dest, 2);
    }

    #[test]
    fn test_new_3() {
        // Given
        let line = "move 11 from 2 to 8";

        // When
        let cmd = Command::new(line);

        // Then
        assert_eq!(cmd.how_many, 11);
        assert_eq!(cmd.source, 1);
        assert_eq!(cmd.dest, 7);
    }

    #[test]
    fn test_new_4() {
        // Given
        let line = "move 15 from 9 to 3";

        // When
        let cmd = Command::new(line);

        // Then
        assert_eq!(cmd.how_many, 15);
        assert_eq!(cmd.source, 8);
        assert_eq!(cmd.dest, 2);
    }
}
