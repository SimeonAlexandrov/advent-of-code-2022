use crate::Environment;

pub fn output(input: &String, env: Environment) {
    use crate::day_five::crates;
    println!("{}", input);
    let crates: Vec<Vec<char>> = crates::get_crates(env);
    println!("crates:{:#?}", crates);

    let line = input.split('\n');
}
