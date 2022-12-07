use regex::Regex;

#[derive(Debug)]
struct Node {}

struct Dir<'a> {
    name: String,
    parent: Option<&'a Dir<'a>>,
    dirs: Vec<&'a Dir<'a>>,
    files: Vec<File>,
}

struct File {
    name: String,
    size: u32,
}
pub fn output(input: &String) {
    let input_iter = input.split('\n');

    let current_dir = Dir {
        name: String::from("init"),
        parent: None,
        dirs: vec![],
        files: vec![],
    };

    let cd_regex = Regex::new("^\\$ cd *").unwrap();
    let ls_regex = Regex::new("^\\$ ls").unwrap();
    let file_regex = Regex::new("^[0-9]* [a-z]*.[a-z]*").unwrap();

    let dir_regex = Regex::new("^dir *").unwrap();
    for line in input_iter {
        if cd_regex.is_match(line) {
            println!("cd found: {}", line);
        }

        if ls_regex.is_match(line) {
            println!("ls found: {}", line);
        }

        if file_regex.is_match(line) {
            println!("file found {}", line);
        }

        if dir_regex.is_match(line) {
            println!("dir found {}", line);
        }
    }
}
