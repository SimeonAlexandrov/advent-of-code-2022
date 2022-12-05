use regex::Regex;
pub struct Command {
    how_many: u8,
    source: u8,
    dest: u8,
}

impl Command {
    pub fn new(s: &str) -> Self {
        let re =
            Regex::new(r"^move ([0-9]|[1-9][0-9]) from ([0-9]|[1-9][0-9]) to ([0-9]|[1-9][0-9])")
                .unwrap();
        let caps = re.captures(s).unwrap();
        println!("{:#?}", caps);
        println!("{}", caps.get(0).unwrap().as_str());
        Self {
            how_many: caps.get(1).unwrap().as_str().parse::<u8>().unwrap(),
            source: caps.get(2).unwrap().as_str().parse::<u8>().unwrap(),
            dest: caps.get(3).unwrap().as_str().parse::<u8>().unwrap(),
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
        assert_eq!(cmd.source, 2);
        assert_eq!(cmd.dest, 1);
    }

    #[test]
    fn test_new_2() {
        // Given
        let line = "move 3 from 1 to 3";

        // When
        let cmd = Command::new(line);

        // Then
        assert_eq!(cmd.how_many, 3);
        assert_eq!(cmd.source, 1);
        assert_eq!(cmd.dest, 3);
    }

    #[test]
    fn test_new_3() {
        // Given
        let line = "move 11 from 2 to 8";

        // When
        let cmd = Command::new(line);

        // Then
        assert_eq!(cmd.how_many, 11);
        assert_eq!(cmd.source, 2);
        assert_eq!(cmd.dest, 8);
    }

    #[test]
    fn test_new_4() {
        // Given
        let line = "move 15 from 9 to 3";

        // When
        let cmd = Command::new(line);

        // Then
        assert_eq!(cmd.how_many, 15);
        assert_eq!(cmd.source, 9);
        assert_eq!(cmd.dest, 3);
    }
}
