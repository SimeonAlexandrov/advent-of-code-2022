use std::cell::RefCell;

#[derive(Debug, Clone, Copy, Default)]
pub enum Op {
    Add,
    Multiply,
    #[default]
    Noop,
}
#[derive(Debug, Clone, Copy, Default)]
pub enum Operand {
    Old,
    Num(usize),
    #[default]
    Noop,
}
#[derive(Debug, Clone, Default)]
pub struct Monkey {
    pub id: usize,
    pub starting_items: Vec<usize>,
    pub items_inspected_cnt: usize,
    pub operation: Op,
    pub operand: Operand,
    pub divisor: Option<usize>,
    pub true_target: Option<usize>,
    pub false_target: Option<usize>,
}

impl Monkey {
    pub fn perform_op(&self, old: &usize) -> usize {
        match self.operation {
            Op::Add => match self.operand {
                Operand::Old => old + old,
                Operand::Num(to_add) => old + to_add,
                Operand::Noop => panic!("Uninit operand"),
            },
            Op::Multiply => match self.operand {
                Operand::Old => old * old,
                Operand::Num(to_mul) => old * to_mul,
                Operand::Noop => panic!("Uninit operand"),
            },
            Op::Noop => panic!("Uninitialized operation"),
        }
    }

    pub fn test(&self, item: usize) -> bool {
        item % self.divisor.unwrap() == 0
    }

    pub fn relieve_worry_level(&self, item: usize) -> usize {
        item / 3 as usize
    }
}
