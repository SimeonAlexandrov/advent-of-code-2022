use std::collections::{HashMap, HashSet};

pub fn output(input: &String, distinct_char_size: usize) {
    let res = find_marker(input, distinct_char_size);
    println!("Result: {}", res.unwrap());
}

fn find_marker(transmission: &String, distinct_char_size: usize) -> Option<usize> {
    let windows = transmission
        .as_bytes()
        .windows(distinct_char_size)
        .enumerate();
    for (i, window) in windows {
        // print!("{:#?}", window);
        let distinct_map: HashSet<u8> = HashSet::from_iter(Vec::from(window));

        // println!("{:#?}", distinct_map);

        // for j in 1..distinct_char_size {
        //     if !distinct_map.contains(&window[j]) {
        //         distinct_map.insert(window[j]);
        //     }
        // }

        if distinct_map.len() == distinct_char_size {
            println!("{:#?}", distinct_map);
            return Some(i + distinct_char_size);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_marker_0() {
        // Given
        let line = String::from("asdfb");

        // When
        let res = find_marker(&line, 4);

        // Then
        assert_eq!(res.unwrap(), 4);
    }

    #[test]
    fn test_find_marker_1() {
        // Given
        let line = String::from("baaaab");

        // When
        let res = find_marker(&line, 4);

        // Then
        assert_eq!(res, None);
    }

    #[test]
    fn test_find_marker_2() {
        // Given
        let line = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");

        // When
        let res = find_marker(&line, 4);

        // Then
        assert_eq!(res.unwrap(), 5);
    }

    #[test]
    fn test_find_marker_3() {
        // Given
        let line = String::from("nppdvjthqldpwncqszvftbrmjlhg");

        // When
        let res = find_marker(&line, 4);

        // Then
        assert_eq!(res.unwrap(), 6);
    }

    #[test]
    fn test_find_marker_4() {
        // Given
        let line = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");

        // When
        let res = find_marker(&line, 4);

        // Then
        assert_eq!(res.unwrap(), 10);
    }

    #[test]
    fn test_find_marker_5() {
        // Given
        let line = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");

        // When
        let res = find_marker(&line, 4);

        // Then
        assert_eq!(res.unwrap(), 11);
    }

    #[test]
    fn test_find_marker_part_two_1() {
        // Given
        let line = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");

        // When
        let res = find_marker(&line, 14);

        // Then
        assert_eq!(res.unwrap(), 19);
    }

    #[test]
    fn test_find_marker_part_two_2() {
        // Given
        let line = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");

        // When
        let res = find_marker(&line, 14);

        // Then
        assert_eq!(res.unwrap(), 23);
    }

    #[test]
    fn test_find_marker_part_two_3() {
        // Given
        let line = String::from("nppdvjthqldpwncqszvftbrmjlhg");

        // When
        let res = find_marker(&line, 14);

        // Then
        assert_eq!(res.unwrap(), 23);
    }

    #[test]
    fn test_find_marker_part_two_4() {
        // Given
        let line = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");

        // When
        let res = find_marker(&line, 14);

        // Then
        assert_eq!(res.unwrap(), 29);
    }

    #[test]
    fn test_find_marker_part_two_5() {
        // Given
        let line = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");

        // When
        let res = find_marker(&line, 14);

        // Then
        assert_eq!(res.unwrap(), 26);
    }
}
