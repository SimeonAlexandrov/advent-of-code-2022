use std::collections::HashMap;
pub mod part_one {
    use std::collections::HashMap;
    #[derive(Debug)]
    struct Backpack {
        compartments: (String, String),
    }

    impl Backpack {
        fn map_backpack_to_common_item(&self) -> Option<char> {
            let mut common_mapper: HashMap<char, char> = HashMap::new();
            let left_compartment = &self.compartments.0;

            for c in left_compartment.chars() {
                common_mapper.insert(c, c);
            }

            println!("Mapper: {:#?}", common_mapper);
            let right_compartment = &self.compartments.1;
            println!("Right: {:#?}", right_compartment);

            for c in right_compartment.chars() {
                if common_mapper.contains_key(&c) == true {
                    return Some(c.clone());
                }
            }
            return None;
        }
    }

    fn split_string(line_string: String) -> (String, String) {
        if line_string.len() % 2 != 0 {
            panic!("String with even length expected!")
        }

        let half = line_string.len() / 2;
        (
            line_string.chars().skip(0).take(half).collect(),
            line_string.chars().skip(half).take(half).collect(),
        )
    }

    pub fn output(input: &String) {
        let backpacks: Vec<Backpack> = input
            .split('\n')
            .map(|line_string| Backpack {
                compartments: split_string(String::from(line_string)),
            })
            .collect();

        println!("{:#?}", backpacks);

        let common_items: Vec<char> = backpacks
            .iter()
            .map(|b| b.map_backpack_to_common_item().unwrap())
            .collect();
        println!("{:#?}", common_items);

        let priorities: Vec<i32> = common_items.iter().map(|ch| match_priority(*ch)).collect();

        println!("{:#?}", priorities);

        println!("Result: {}", priorities.iter().sum::<i32>())
    }

    fn match_priority(ch: char) -> i32 {
        match ch {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            'i' => 9,
            'j' => 10,
            'k' => 11,
            'l' => 12,
            'm' => 13,
            'n' => 14,
            'o' => 15,
            'p' => 16,
            'q' => 17,
            'r' => 18,
            's' => 19,
            't' => 20,
            'u' => 21,
            'v' => 22,
            'w' => 23,
            'x' => 24,
            'y' => 25,
            'z' => 26,
            'A' => 27,
            'B' => 28,
            'C' => 29,
            'D' => 30,
            'E' => 31,
            'F' => 32,
            'G' => 33,
            'H' => 34,
            'I' => 35,
            'J' => 36,
            'K' => 37,
            'L' => 38,
            'M' => 39,
            'N' => 40,
            'O' => 41,
            'P' => 42,
            'Q' => 43,
            'R' => 44,
            'S' => 45,
            'T' => 46,
            'U' => 47,
            'V' => 48,
            'W' => 49,
            'X' => 50,
            'Y' => 51,
            'Z' => 52,
            _ => panic!("Unexpected priority"),
        }
    }

    #[cfg(test)]
    mod tests {
        // Note this useful idiom: importing names from outer (for mod tests) scope.
        use super::*;

        #[test]
        fn test_split_string() {
            assert_eq!(
                split_string(String::from("abab")),
                (String::from("ab"), String::from("ab"))
            );
            assert_eq!(
                split_string(String::from("aaaaaaaa")),
                (String::from("aaaa"), String::from("aaaa"))
            );
            assert_eq!(
                split_string(String::from("vJrwpWtwJgWrhcsFMMfFFhFp")),
                (String::from("vJrwpWtwJgWr"), String::from("hcsFMMfFFhFp"))
            );
            assert_eq!(
                split_string(String::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL")),
                (
                    String::from("jqHRNqRjqzjGDLGL"),
                    String::from("rsFMfFZSrLrFZsSL")
                )
            );
        }

        #[test]
        #[should_panic(expected = "String with even length expected!")]
        fn test_split_string_uneven() {
            split_string(String::from("abc"));
        }

        #[test]
        fn test_map_backpack_to_common_item() {
            let b = Backpack {
                compartments: (String::from("asdf"), String::from("zxca")),
            };

            assert_eq!(b.map_backpack_to_common_item().unwrap(), 'a');
        }

        #[test]
        fn test_map_backpack_to_common_item_dummy1() {
            let b = Backpack {
                compartments: (String::from("vJrwpWtwJgWr"), String::from("hcsFMMfFFhFp")),
            };

            assert_eq!(b.map_backpack_to_common_item().unwrap(), 'p');
        }

        #[test]
        fn test_map_backpack_to_common_item_dummy2() {
            let b = Backpack {
                compartments: (String::from("PmmdzqPrV"), String::from("vPwwTWBwg")),
            };

            assert_eq!(b.map_backpack_to_common_item().unwrap(), 'P');
        }
    }
}

pub mod part_two {
    pub fn output(input: &String) {
        println!("{}", input)
    }
}
