use crate::day_seven::dir::Dir;
use crate::day_seven::line::parse_line;
// We need some common trait for Dir and File getSize
// It's going to be implemented recursively for Dirs

pub fn output(input: &String) {
    let input_iter = input.split('\n');

    let mut current_dir = Dir {
        name: String::from("/"),
        parent: None,
        dirs: vec![],
        files: vec![],
    };

    for line in input_iter {
        parse_line(line, &mut current_dir);
    }
}
