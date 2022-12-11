pub struct Monkey {
    id: usize,
    starting_items: Vec<usize>,
    items_inspected_cnt: usize
}

impl Monkey {
    fn operation(&self, old: &usize, op: &char, operand: &usize) -> usize {
        match op {
            '+' => old + operand,
            '*' => old * operand,
            _ => panic!("Unknown operation!"),
        }
    }

    fn test(&self, item: &usize, operand: &usize) -> {
        item % operand == 0
    }

    fn relieve_worry_level(&self, item: &usize) -> usize {
        item / 3 as usize
    }
}
