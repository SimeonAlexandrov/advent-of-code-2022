// We need some common trait for Dir and File getSize
// It's going to be implemented recursively for Dirs

use regex::Regex;

pub fn output(input: &String) {
    let input_iter = input.split('\n');

    for line in input_iter {
        parse_line(line);
    }
}

pub fn parse_line(line: &str) {
    let cd_regex = Regex::new("^\\$ cd ([a-z]*)").unwrap();
    let ls_regex = Regex::new("^\\$ ls").unwrap();
    let file_regex = Regex::new("^([0-9]*) (([a-z]*).[a-z]*|([a-z]*))").unwrap();
    let dir_regex = Regex::new("^dir ([a-z]*)").unwrap();

    if cd_regex.is_match(line) {
        let caps = cd_regex.captures(line).unwrap();
        println!("cd target: {:#?}", caps.get(1).unwrap().as_str());
    }

    if ls_regex.is_match(line) {
        println!("ls found: {}", line);
    }

    if file_regex.is_match(line) {
        let caps = file_regex.captures(line).unwrap();
        println!(
            "original line: {:#?} \n\tfile size: {:#?}  \n\tfile_name: {:#?}",
            line,
            caps.get(1).unwrap().as_str(),
            caps.get(2).unwrap().as_str()
        );
    }

    if dir_regex.is_match(line) {
        let caps = dir_regex.captures(line).unwrap();
        println!(
            "original line: {} \n\tdir_name: {:#?}",
            line,
            caps.get(1).unwrap().as_str()
        );
    }
}
