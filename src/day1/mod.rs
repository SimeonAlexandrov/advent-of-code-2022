pub mod solution {
    pub fn output(input: &String) {
        let calories_vec: Vec<i32> = input
            .split("\n")
            .map(|s| match s.parse::<i32>() {
                Ok(num) => num,
                Err(_) => -1,
            })
            .collect();

        println!("{:#?}", calories_vec);
    }
}
