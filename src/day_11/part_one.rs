use std::{cell::RefCell, rc::Rc};

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
                println!("New monkey: {:#?}", &current_monkey);
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
                    println!("Current monkey: {:#?}", &current_monkey);
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
                    println!("Current monkey: {:#?}", &current_monkey);
                }
                _ => panic!("Unknown word!"),
            },
            "Test:" => match words[5] {
                num => (*current_monkey).borrow_mut().divisor = Some(num.parse::<usize>().unwrap()),
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
                println!("Current monkey: {:#?}", (*current_monkey).borrow());
            }
            _ => {
                println!("{:#?}", words);
                panic!("Unknown word!")
            }
        }
    }
    println!("Monkeys: {:?}", &monkeys);
}

// for monkey in monkeys {
//     for item in &monkey.as_ref().unwrap().starting_items {
//         let mut worry_level = monkey.as_ref().unwrap().perform_op(&item);
//         println!("New worry level: {}", worry_level);

//         // Relieve stress
//         worry_level = monkey.as_ref().unwrap().relieve_worry_level(worry_level);
//         println!("New worry level after relieved stress: {}", worry_level);

//         // Test item
//         if monkey.as_ref().unwrap().test(worry_level) {
//             monkeys
//                 .get_mut(monkey.as_ref().unwrap().true_target.unwrap())
//                 .unwrap()
//                 .as_mut()
//                 .unwrap()
//                 .starting_items
//                 .push(worry_level);
//         }
//     }
// }
// Monkey's turn:
// - inspect
// - throw all items left to right

// Each monkey one turn

// Item goes to end of list
