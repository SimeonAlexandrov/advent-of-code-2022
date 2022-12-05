use crate::Environment;

pub fn output(input: &String, env: &Environment) {
    use crate::day_five::command::Command;
    use crate::day_five::crates;
    use crate::day_five::stack::Stack;
    // println!("{}", input);
    let mut stacks: Vec<Stack> = crates::get_crates(env)
        .iter()
        .map(|v| Stack {
            content: v.to_vec(),
        })
        .collect();
    // println!("crates:{:#?}", stacks);

    let commands: Vec<Command> = input.split('\n').map(|line| Command::new(line)).collect();

    // println!("commands: {:#?}", commands);

    for cmd in commands.iter() {
        // Pop
        let source_stack = stacks.get_mut(cmd.source).unwrap();
        let mut top_value = source_stack.pop_v2(cmd.how_many).unwrap();

        // Push
        let dest_stack = stacks.get_mut(cmd.dest).unwrap();
        dest_stack.push_v2(&mut top_value);

        println!("{:#?}", stacks)
    }

    for st in stacks {
        print!("{}", st.peek().unwrap());
    }
    println!()
}
