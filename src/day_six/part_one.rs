pub fn output(input: &String) {
    println!("{input}");
    let res = find_marker(input);
    println!("Result: {}", res.unwrap());
}

fn find_marker(transmission: &String) -> Option<usize> {
    let windows = transmission.as_bytes().windows(4).enumerate();
    for (i, window) in windows {
        // print!("{:#?}", window);
        if window[0] != window[1]
            && window[0] != window[2]
            && window[0] != window[3]
            && window[1] != window[2]
            && window[1] != window[3]
            && window[2] != window[3]
        {
            return Some(i + 4);
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
        let res = find_marker(&line);

        // Then
        assert_eq!(res.unwrap(), 4);
    }

    #[test]
    fn test_find_marker_1() {
        // Given
        let line = String::from("baaaab");

        // When
        let res = find_marker(&line);

        // Then
        assert_eq!(res, None);
    }

    #[test]
    fn test_find_marker_2() {
        // Given
        let line = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");

        // When
        let res = find_marker(&line);

        // Then
        assert_eq!(res.unwrap(), 5);
    }

    #[test]
    fn test_find_marker_3() {
        // Given
        let line = String::from("nppdvjthqldpwncqszvftbrmjlhg");

        // When
        let res = find_marker(&line);

        // Then
        assert_eq!(res.unwrap(), 6);
    }

    #[test]
    fn test_find_marker_4() {
        // Given
        let line = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");

        // When
        let res = find_marker(&line);

        // Then
        assert_eq!(res.unwrap(), 10);
    }

    #[test]
    fn test_find_marker_5() {
        // Given
        let line = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");

        // When
        let res = find_marker(&line);

        // Then
        assert_eq!(res.unwrap(), 11);
    }
}
