pub mod solution {
    pub fn output(input: &String) {
        let calories_vec: Vec<i32> = input
            .split("\n")
            .map(|s| match s.parse::<i32>() {
                Ok(num) => num,
                Err(_) => -1,
            })
            .collect();

        let mut elf_with_most_calories = 0;
        let mut current_elf = 0;
        for cal in calories_vec.iter() {
            if *cal == -1 {
                // Check if current elf is better than best
                elf_with_most_calories = update_best_elf(current_elf, elf_with_most_calories);
                current_elf = 0;
            } else {
                current_elf += cal;
            }
        }
        // Check for last sum as well
        elf_with_most_calories = update_best_elf(current_elf, elf_with_most_calories);

        println!("Result: {elf_with_most_calories}")
    }

    fn update_best_elf(current: i32, best: i32) -> i32 {
        let new_best = match current > best {
            true => current,
            _ => best,
        };
        new_best
    }
}
