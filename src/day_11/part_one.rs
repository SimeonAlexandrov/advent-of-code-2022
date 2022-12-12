use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::day_11::monkey::{Op, Operand};

pub fn output(input: &String) {
    use super::monkey::Monkey;
    // println!("{input}");

    let lines = input.lines();

    let mut monkeys: Vec<&Monkey> = vec![];

    let mut current_monkey: Option<Box<Monkey>> = None;

    for line in lines {
        let words = line.split(" ").collect::<Vec<&str>>();
        println!("{:#?}", words);
        if line.len() == 0 {
            continue;
        }
        match words[0] {
            "Monkey" => {
                let new_monkey = Box::from(Monkey {
                    id: words[1]
                        .split(":")
                        .collect::<Vec<&str>>()
                        .get(0)
                        .unwrap()
                        .parse::<usize>()
                        .unwrap(),
                    starting_items: vec![],
                    items_inspected_cnt: 0,
                    operation: None,
                    operand: None,
                    divisor: None,
                    true_target: None,
                    false_target: None,
                });
                current_monkey = Some(new_monkey);
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

                        match current_monkey.as_mut() {
                            Some(current_monkey) => current_monkey.starting_items.push(parsed),
                            None => panic!("Current monkey is not set!"),
                        }
                    }
                    println!("Current monkey: {:#?}", &current_monkey);
                }
                "Operation:" => {
                    match current_monkey.as_mut() {
                        Some(current_monkey) => match words[6] {
                            "+" => current_monkey.operation = Some(Op::Add),
                            "*" => current_monkey.operation = Some(Op::Multiply),
                            _ => panic!("Unknown operation!"),
                        },
                        None => println!("Current monkey is not set!"),
                    };
                    match current_monkey.as_mut() {
                        Some(current_monkey) => match words[7] {
                            "old" => current_monkey.operand = Some(Operand::Old),
                            other => {
                                current_monkey.operand =
                                    Some(Operand::Num(other.parse::<usize>().unwrap()))
                            }
                        },
                        None => println!("Current monkey is not set!"),
                    };
                    println!("Current monkey: {:#?}", &current_monkey);
                }
                "Test:" => match current_monkey.as_mut() {
                    Some(current_monkey) => match words[5] {
                        num => current_monkey.divisor = Some(num.parse::<usize>().unwrap()),
                    },
                    None => println!("Current monkey is not set!"),
                },
                "" => {
                    match current_monkey.as_mut() {
                        Some(current_monkey) => match words[5] {
                            "true:" => {
                                current_monkey.true_target =
                                    Some(words[9].parse::<usize>().unwrap())
                            }
                            "false:" => {
                                current_monkey.false_target =
                                    Some(words[9].parse::<usize>().unwrap())
                            }
                            unknown => {
                                panic!("Unknown word in indented line! {unknown}")
                            }
                        },
                        None => println!("Current monkey is not set!"),
                    };
                    println!("Current monkey: {:#?}", &current_monkey);
                }
                unknown => {
                    panic!("Unknown word in indented line! {unknown}")
                }
            },
            _ => {
                println!("{:#?}", words);
                panic!("Unknown word!")
            }
        }
        // match monkeys.get_mut(current_monkey.as_ref().unwrap().id) {
        //     Some(monkey) => *monkey = current_monkey.as_ref().unwrap(),
        //     None => monkeys.push(current_monkey.as_ref().unwrap()),
        // }
    }
    println!("Monkeys: {:#?}", &monkeys)

    // Monkey's turn:
    // - inspect
    // - throw all items left to right

    // Each monkey one turn

    // Item goes to end of list
}
