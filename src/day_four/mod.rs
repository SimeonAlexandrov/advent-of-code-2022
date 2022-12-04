pub mod assignment;
pub mod part_one {
    use crate::day_four::assignment::Assignment;

    fn parse_str(s: &str) -> Assignment {
        let mut it = s.split('-');
        Assignment(
            it.next().unwrap().parse::<u8>().unwrap(),
            it.next().unwrap().parse::<u8>().unwrap(),
        )
    }

    pub fn get_assignments(input: &String) -> Vec<(Assignment, Assignment)> {
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
        return assignments_in_line;
    }

    pub fn output(input: &String) {
        let assignments_in_line: Vec<(Assignment, Assignment)> = get_assignments(input);

        let mut contain_counter: u16 = 0;

        for assignments in &assignments_in_line {
            let first = assignments.0;
            let second = assignments.1;
            if first.contains(&second) || second.contains(&first) {
                contain_counter += 1;
            }
        }

        println!("Result: {}", contain_counter);
    }
}

pub mod part_two {
    use crate::day_four::part_one::get_assignments;

    pub fn output(input: &String) {
        let assignments_in_line = get_assignments(input);

        let mut overlap_counter: u16 = 0;

        for assignments in &assignments_in_line {
            let first = assignments.0;
            let second = assignments.1;
            if first.overlaps(&second) || second.overlaps(&first) {
                overlap_counter += 1;
            }
        }

        println!("Result: {}", overlap_counter);
    }
}
