#[allow(dead_code)]
pub mod part_one {
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

        println!("Result 1: {elf_with_most_calories}")
    }

    fn update_best_elf(current: i32, best: i32) -> i32 {
        let new_best = match current > best {
            true => current,
            _ => best,
        };
        new_best
    }
}

#[allow(dead_code)]
pub mod part_two {
    pub fn output(input: &String) {
        let calories_vec: Vec<i32> = input
            .split("\n")
            .map(|s| match s.parse::<i32>() {
                Ok(num) => num,
                Err(_) => -1,
            })
            .collect();

        let mut elf_ranking: Vec<i32> = Vec::new();

        let mut current_elf = 0;
        for cal in calories_vec.iter() {
            if *cal == -1 {
                println!("Current elf: {current_elf}");
                elf_ranking.push(current_elf);
                current_elf = 0;
            } else {
                current_elf += cal;
            }
        }
        println!("Final current elf: {current_elf}");
        elf_ranking.push(current_elf);

        println!("Ranking: {:#?}", elf_ranking);
        elf_ranking.sort();
        elf_ranking.reverse();
        println!("Sorted ranking: {:#?}", elf_ranking);

        let result = &elf_ranking[0..3];
        println!("Result {:#?}", result.iter().sum::<i32>());
    }
}
