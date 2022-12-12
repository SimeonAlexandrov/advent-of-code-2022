use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    rc::Rc,
};

use crate::day_11::monkey::{Op, Operand};

pub fn output(input: &String) {
    use super::monkey::Monkey;
    // println!("{input}");

    let lines = input.lines();

    let monkeys: Vec<Monkey> = vec![];

    let mut current_monkey: Option<Box<Monkey>> = None;

    for line in lines {
        let words = line.split(" ").collect::<Vec<&str>>();
        println!("{:#?}", words);
        match words[0] {
            "Monkey" => {
                let mut new_monkey = Box::from(Monkey {
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
                });
                current_monkey = Some(new_monkey);
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
                            Option::Some(current_monkey) => {
                                current_monkey.starting_items.push(parsed)
                            }
                            Option::None => panic!("Current monkey is not set!"),
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
                unknown => {
                    panic!("Unknown word in indented line! {unknown}")
                }
            },
            _ => {
                println!("{:#?}", words);
                panic!("Unknown word!")
            }
        }
    }

    // Monkey's turn:
    // - inspect
    // - throw all items left to right

    // Each monkey one turn

    // Item goes to end of list
}
