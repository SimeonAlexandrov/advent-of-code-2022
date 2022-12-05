#[derive(Debug)]
pub struct Stack {
    pub content: Vec<char>,
}

impl Stack {
    pub fn push(&mut self, ch: &char) -> Option<()> {
        Some(self.content.push(*ch))
    }

    pub fn push_v2(&mut self, containers: &mut Vec<char>) -> Option<()> {
        Some(self.content.append(containers))
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.content.len() == 0 {
            panic!("Failed popping from empty stack!");
        }
        self.content.pop()
    }

    pub fn pop_v2(&mut self, how_many: usize) -> Option<Vec<char>> {
        if self.content.len() < how_many {
            panic!("Failed popping from empty or not long enough stack!");
        }

        Some(self.content.drain(..how_many).collect())
    }

    pub fn peek(self) -> Option<char> {
        if self.content.len() == 0 {
            panic!("Failed peeking from empty stack!");
        }
        Some(self.content[self.content.len() - 1])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod push {
        use super::*;
        #[test]
        fn test_push() {
            // Given
            let mut s = Stack {
                content: vec!['A', 'B'],
            };
            // When
            s.push(&'C');
            // Then
            assert_eq!(s.content, vec!['A', 'B', 'C']);
        }
        #[test]
        fn test_push_empty() {
            // Given
            let mut s = Stack { content: vec![] };
            // When
            s.push(&'A');
            // Then
            assert_eq!(s.content, vec!['A']);
        }
    }

    mod push_v2 {
        use super::*;
        #[test]
        fn test_push_v2_single() {
            // Given
            let mut s = Stack {
                content: vec!['A', 'B'],
            };
            // When
            s.push_v2(&mut vec!['C']);
            // Then
            assert_eq!(s.content, vec!['A', 'B', 'C']);
        }

        #[test]
        fn test_push_v2_multiple() {
            // Given
            let mut s = Stack {
                content: vec!['A', 'B'],
            };
            // When
            s.push_v2(&mut vec!['C', 'D']);
            // Then
            assert_eq!(s.content, vec!['A', 'B', 'C', 'D']);
        }

        #[test]
        fn test_push_v2_empty() {
            // Given
            let mut s = Stack { content: vec![] };
            // When
            s.push_v2(&mut vec!['A', 'B']);
            // Then
            assert_eq!(s.content, vec!['A', 'B']);
        }
    }

    mod pop {
        use super::*;

        #[test]
        fn test_pop() {
            // Given
            let mut s = Stack {
                content: vec!['A', 'B'],
            };
            // When
            s.pop();
            // Then
            assert_eq!(s.content, vec!['A']);
        }

        #[test]
        #[should_panic(expected = "Failed popping from empty stack!")]
        fn test_pop_empty() {
            // Given
            let mut s = Stack { content: vec![] };
            // When
            s.pop();
        }
    }

    mod pop_v2 {
        use super::*;

        #[test]
        fn test_pop_v2() {
            // Given
            let mut s = Stack {
                content: vec!['A', 'B'],
            };
            let empty_vec: Vec<char> = Vec::new();
            // When
            // Then
            let res = s.pop_v2(2).unwrap().clone();
            assert_eq!(res, &['A', 'B']);

            assert_eq!(s.content, empty_vec);
        }

        #[test]
        #[should_panic(expected = "Failed popping from empty or not long enough stack!")]
        fn test_pop_v2_empty() {
            // Given
            let mut s = Stack { content: vec![] };
            // When
            s.pop_v2(2);
        }

        #[test]
        #[should_panic(expected = "Failed popping from empty or not long enough stack!")]
        fn test_pop_v2_not_long_enough() {
            // Given
            let mut s = Stack { content: vec!['A'] };
            // When
            s.pop_v2(2);
        }
    }

    mod peek {
        use super::*;

        #[test]
        fn test_peek() {
            // Given
            let s = Stack {
                content: vec!['A', 'B'],
            };
            // When
            let res = s.peek().unwrap();
            // Then
            assert_eq!(res, 'B');
        }

        #[test]
        #[should_panic(expected = "Failed peeking from empty stack!")]
        fn test_peek_empty() {
            // Given
            let s = Stack { content: vec![] };
            // When
            s.peek().unwrap();
        }
    }
}
