struct Stack {
    content: Vec<char>,
}

impl Stack {
    pub fn push(&mut self, ch: &char) -> Option<()> {
        Some(self.content.push(*ch))
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.content.len() == 0 {
            panic!("Failed popping from empty stack!");
        }
        self.content.pop()
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
