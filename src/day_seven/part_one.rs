use std::cell::RefCell;
use std::{collections::HashMap, rc::Rc};
#[derive(Default)]
struct Dir {
    name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Dir>>,
    subdir: RefCell<HashMap<String, Rc<Dir>>>,
}

impl Dir {
    fn get_size(&self) -> usize {
        *self.size.borrow()
            + self
                .subdir
                .borrow()
                .values()
                .fold(0, |a, b| a + b.get_size())
    }
}

pub fn output(input: &String) {
    println!("{input}");
    let root = Rc::new(Dir {
        name: String::from("/"),
        size: RefCell::new(0),
        parent: None,
        subdir: RefCell::new(HashMap::new()),
    });
    let mut cwd = Rc::clone(&root);
    for line in input.split('\n') {
        let words = line.split(' ').collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match words[2] {
                "/" => cwd = Rc::clone(&root),
                ".." => cwd = Rc::clone(cwd.parent.as_ref().unwrap()),
                dirname => {
                    let newdir = cwd.subdir.borrow().get(dirname).unwrap().clone();
                    cwd = Rc::clone(&newdir);
                }
            },
            ("dir", dirname) => {
                cwd.subdir.borrow_mut().insert(
                    dirname.to_string(),
                    Rc::new(Dir {
                        name: dirname.to_string(),
                        size: RefCell::new(0),
                        parent: Some(Rc::clone(&cwd)),
                        subdir: RefCell::new(HashMap::new()),
                    }),
                );
            }

            (size, name) => {
                *cwd.size.borrow_mut() += size.parse::<usize>().unwrap();
            }
        }
    }

    let mut to_visit = vec![Rc::clone(&root)];
    let mut total = 0;
    let max_size = 70000000;
    let mut min_acceptable_dir_size = max_size;
    let total_size_root = root.get_size();
    while let Some(dir) = to_visit.pop() {
        for d in dir.subdir.borrow().values() {
            to_visit.push(Rc::clone(d));
        }

        let size = dir.get_size();
        if size <= 100000 {
            total += size;
        }

        if (max_size - total_size_root + size) > 30000000 && size < min_acceptable_dir_size {
            min_acceptable_dir_size = size;
        }
    }
    println!("Total: {total}");

    // println!("Total size root: {}", total_size_root);
    println!("Min acceptable dir size {}", min_acceptable_dir_size);
}
