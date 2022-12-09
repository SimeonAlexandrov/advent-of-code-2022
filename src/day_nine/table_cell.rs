#[derive(Debug)]
pub enum IsVisited {
    Visited,
    NotVisited,
}
#[derive(Debug)]
pub enum CellType {
    Visited,
    NotVisited,
    Head,
    Tail,
    HeadTail,
}

#[derive(Debug)]
pub struct TableCell {
    pub is_visited: IsVisited,
    pub cell_type: CellType,
}

impl TableCell {
    pub fn new() -> &'static Self {
        &Self {
            is_visited: IsVisited::NotVisited,
            cell_type: CellType::NotVisited,
        }
    }

    pub fn print(&self) -> char {
        match self.cell_type {
            CellType::Visited => '#',
            CellType::NotVisited => '.',
            CellType::Head => 'H',
            CellType::Tail => 'T',
            CellType::HeadTail => 'H', // When overlapped print H
        }
        // print!("{}", str_representation);
    }

    pub fn visit(&mut self) {
        self.is_visited = IsVisited::Visited;
    }
}
