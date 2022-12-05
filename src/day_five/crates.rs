use std::vec;

use crate::Environment;

pub fn get_crates(env: Environment) -> Vec<Vec<char>> {
    match env {
        Environment::Dummy => vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']],
        Environment::Submission => vec![
            vec!['H', 'B', 'V', 'W', 'N', 'M', 'L', 'P'],
            vec!['M', 'Q', 'H'],
            vec!['N', 'D', 'B', 'G', 'F', 'Q', 'M', 'L'],
            vec!['Z', 'T', 'F', 'Q', 'M', 'W', 'G'],
            vec!['M', 'T', 'H', 'P'],
            vec!['C', 'B', 'M', 'J', 'D', 'H', 'G', 'T'],
            vec!['M', 'N', 'B', 'F', 'V', 'R'],
            vec!['P', 'L', 'H', 'M', 'R', 'G', 'S'],
            vec!['P', 'D', 'B', 'C', 'N'],
        ],
    }
}
