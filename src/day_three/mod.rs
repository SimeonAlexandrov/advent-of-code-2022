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

        let common_items = backpacks
            .iter()
            .map(|b| b.map_backpack_to_common_item().unwrap());
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
