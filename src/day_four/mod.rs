pub mod assignment;
pub mod part_one {
    // use core::slice::Split
    // use crate::day_four::assignment::Assignment;

    use crate::day_four::assignment::Assignment;

    pub fn parse_str(s: &str) -> Assignment {
        let mut it = s.split('-');
        Assignment(
            it.next().unwrap().parse::<u8>().unwrap(),
            it.next().unwrap().parse::<u8>().unwrap(),
        )
    }

    pub fn output(input: &String) {
        let assignments_in_line: Vec<(Assignment, Assignment)> = input
            .split('\n')
            .map(|line| {
                let mut line_split = line.split(',');
                let first_assignment = line_split.next().unwrap();
                let second_assignment = line_split.next().unwrap();
                return (first_assignment, second_assignment);
            })
            .map(|str_assignments| (parse_str(str_assignments.0), parse_str(str_assignments.1)))
            .collect();
        println!("{:#?}", assignments_in_line);
    }
}
