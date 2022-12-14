use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::day_11::monkey::{Op, Operand};

pub fn output(input: &String) {
    use super::monkey::Monkey;
    // println!("{input}");

    let lines = input.lines();

    let mut monkeys: Vec<Rc<RefCell<Monkey>>> = vec![];

    let mut current_monkey: Rc<RefCell<Monkey>> = Rc::new(RefCell::new(Monkey::default()));

    for line in lines {
        let words = line.split(" ").collect::<Vec<&str>>();
        println!("{:#?}", words);
        if line.len() == 0 {
            continue;
        }
        match words[0] {
            "Monkey" => {
                let new_monkey = Rc::new(RefCell::from(Monkey {
                    id: words[1]
                        .split(":")
                        .collect::<Vec<&str>>()
                        .get(0)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                    starting_items: vec![],
                    items_inspected_cnt: 0,
                    operation: Op::Noop,
                    operand: Operand::Noop,
                    divisor: None,
                    true_target: None,
                    false_target: None,
                }));
                current_monkey = Rc::clone(&new_monkey);
                // monkeys.push(current_monkey.as_ref().unwrap());
                // println!("New monkey: {:#?}", &current_monkey);
            }
            "" => match words[2] {
                "Starting" => {
                    let items = &words[4..words.len()]
                        .iter()
                        .map(|item| item.to_string())
                        .collect::<Vec<String>>();

                    for mut item in items {
                        let without_comma_item = item.replace(",", "");
                        let parsed = without_comma_item.parse::<usize>().unwrap();

                        (*current_monkey).borrow_mut().starting_items.push(parsed)
                    }
                    // println!("Current monkey: {:#?}", &current_monkey);
                }
                "Operation:" => {
                    match words[6] {
                        "+" => (*current_monkey).borrow_mut().operation = Op::Add,
                        "*" => (*current_monkey).borrow_mut().operation = Op::Multiply,
                        _ => panic!("Unknown operation!"),
                    };
                    match words[7] {
                        "old" => (*current_monkey).borrow_mut().operand = Operand::Old,
                        other => {
                            (*current_monkey).borrow_mut().operand =
                                Operand::Num(other.parse::<usize>().unwrap())
                        }
                    };
                    // println!("Current monkey: {:#?}", &current_monkey);
                }
                "Test:" => match words[5] {
                    num => {
                        (*current_monkey).borrow_mut().divisor = Some(num.parse::<usize>().unwrap())
                    }
                },
                "" => {
                    match words[5] {
                        "true:" => {
                            (*current_monkey).borrow_mut().true_target =
                                Some(words[9].parse::<usize>().unwrap())
                        }
                        "false:" => {
                            (*current_monkey).borrow_mut().false_target =
                                Some(words[9].parse::<usize>().unwrap());
                            monkeys.push(Rc::clone(&current_monkey))
                        }
                        unknown => {
                            panic!("Unknown word in indented line! {unknown}")
                        }
                    };
                    // println!("Current monkey: {:#?}", (*current_monkey).borrow());
                }
                _ => {
                    println!("{:#?}", words);
                    panic!("Unknown indented word!")
                }
            },
            _ => {
                println!("{:#?}", words);
                panic!("Unknown word!")
            }
        }
    }
    println!("Monkeys: {:#?}", &monkeys);

    let mut inspected_items: HashMap<usize, usize> = HashMap::new();

    for i in 1..=20 {
        println!("============");
        for m in &monkeys {
            println!(
                "Monkey: {}, {:?}",
                (*m).borrow().id,
                (*m).borrow().starting_items
            )
        }
        for monkey in &monkeys {
            // inspected_items
            //     .entry((*monkey).borrow().id)
            //     .and_modify(|e| *e += 1)
            //     .or_insert(0);
            println!(
                "Monkey {} starting items before round: {:?}",
                (*monkey).borrow().id,
                (*monkey).borrow().starting_items
            );
            (*monkey).borrow().starting_items.iter().for_each(|item| {
                // Count towards inspected items
                inspected_items
                    .entry((*monkey).borrow().id)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);

                let mut worry_level = (*monkey).borrow().perform_op(item);
                println!("\tNew worry level: {}", worry_level);

                // Relieve stress
                worry_level = (*monkey).borrow().relieve_worry_level(worry_level);
                println!("\tNew worry level after relieved stress: {}", worry_level);

                // Test item
                if (*monkey).borrow().test(worry_level) {
                    let target_id = (*monkey).borrow().true_target.unwrap();
                    println!("\tNeed to pass {} to monkey {:?}", worry_level, target_id);
                    monkeys[target_id]
                        .borrow_mut()
                        .starting_items
                        .push(worry_level)
                } else {
                    let target_id = (*monkey).borrow().false_target.unwrap();
                    println!(
                        "\tNeed to pass {} to monkey {:?} cause not divisible",
                        worry_level,
                        (*monkey).borrow().false_target.unwrap()
                    );

                    monkeys[target_id]
                        .borrow_mut()
                        .starting_items
                        .push(worry_level)
                }
                // (*monkey).borrow_mut().items_inspected_cnt += 1;
            });
            (*monkey).borrow_mut().starting_items = vec![];
        }
        println!("Round i: {}\tInspected items {:?}", i, &inspected_items);
    }
    // println!("Monkeys: {:#?}", &monkeys);
    // println!("Inspected items {:?}", &inspected_items);
}

// Monkey's turn:
// - inspect
// - throw all items left to right

// Each monkey one turn

// Item goes to end of list
