pub struct Monkey {
    id: usize,
    starting_items: Vec<usize>,
}

impl Monkey {
    fn operation(&self, old: &usize, op: &char, operand: &usize) -> usize {
        match op {
            '+' => old + operand,
            '*' => old * operand,
            _ => panic!("Unknown operation!"),
        }
    }
}
