#[derive(Debug)]
pub struct Assignment(pub u8, pub u8);

impl Assignment {
    pub fn contains(self, other: &Assignment) -> bool {
        self.0 >= other.0 && self.1 <= other.1
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_contains_0() {
        assert_eq!(Assignment(2, 4).contains(&Assignment(6, 8)), false,);
    }

    #[test]
    fn test_contains_1() {
        assert_eq!(Assignment(2, 3).contains(&Assignment(4, 5)), false);
    }

    #[test]
    fn test_contains_2() {
        assert_eq!(Assignment(2, 8).contains(&Assignment(3, 7)), false);
    }

    #[test]
    fn test_contains_3() {
        assert_eq!(Assignment(25, 96).contains(&Assignment(3, 96)), true);
    }

    #[test]
    fn test_contains_4() {
        assert_eq!(Assignment(7, 8).contains(&Assignment(11, 95)), false);
    }

    #[test]
    fn test_contains_5() {
        assert_eq!(Assignment(3, 7).contains(&Assignment(2, 8)), true);
    }
}
