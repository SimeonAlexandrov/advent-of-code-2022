// We need some common trait for Dir and File getSize
// It's going to be implemented recursively for Dirs

use std::cell::Cell;

use regex::Regex;

enum NodeType {
    File,
    Dir,
}

struct Node<'a> {
    name: String,
    node_type: NodeType,
    size: Cell<u32>,
    parent: Option<Box<&'a Node<'a>>>,
    children: Vec<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    fn register_child(&mut self, child: Box<Node<'a>>) {
        self.children.push(child);
    }
}

pub fn output(input: &String) {
    let input_iter = input.split('\n');

    let mut root = Node {
        name: String::from('/'),
        node_type: NodeType::Dir,
        size: Cell::new(0),
        parent: None,
        children: vec![],
    };

    let mut a = Node {
        name: String::from('a'),
        node_type: NodeType::Dir,
        size: Cell::new(0),
        parent: Some(Box::new(&root)),
        children: vec![],
    };

    root.register_child(Box::new(a));

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
