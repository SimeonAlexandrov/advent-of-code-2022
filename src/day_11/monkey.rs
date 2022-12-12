use std::cell::RefCell;

#[derive(Debug)]
pub enum Op {
    Add,
    Multiply,
}
#[derive(Debug)]
pub enum Operand {
    Old,
    Num(usize),
}
#[derive(Debug)]
pub struct Monkey {
    pub id: usize,
    pub starting_items: Vec<usize>,
    pub items_inspected_cnt: usize,
    pub operation: Option<Op>,
    pub operand: Option<Operand>,
}

impl Monkey {
    fn operation(&self, old: &usize, op: Op, operand: Operand) -> usize {
        match op {
            Op::Add => match operand {
                Operand::Old => old + old,
                Operand::Num(to_add) => old + to_add,
            },
            Op::Multiply => match operand {
                Operand::Old => old * old,
                Operand::Num(to_mul) => old * to_mul,
            },
        }
    }

    fn test(&self, item: &usize, operand: &usize) -> bool {
        item % operand == 0
    }

    fn relieve_worry_level(&self, item: &usize) -> usize {
        item / 3 as usize
    }
}
