use std::ops::Deref;

use super::table_cell::{CellType, IsVisited, TableCell};

pub struct Table<'a> {
    content: Vec<Vec<&'a TableCell>>,
    head: (usize, usize),
    tail: (usize, usize),
}

impl<'a> Table<'a> {
    pub fn new(y: usize, x: usize) -> Self {
        let mut self_obj = Self {
            content: vec![vec![TableCell::new(); x]; y],
            head: (y / 2, x / 2),
            tail: (y / 2, x / 2),
        };
        let cell = self_obj.content[y].get_mut(x).unwrap();
        // cell.cell_type = CellType::HeadTail;
        let c = cell. = IsVisited::Visited;
        self_obj
    }

    pub fn print(self) {
        for row in self.content.iter() {
            for cell in row.iter() {
                print!("{:#?}", cell.print());
            }
        }
    }
}
