use pathfinding::prelude::dijkstra;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Default)]
struct PosYX(i32, i32);

type Table = Vec<Vec<i32>>;

impl PosYX {
    pub fn successors(&self, table: &Table) -> Vec<(PosYX, usize)> {
        let mut successors: Vec<(PosYX)> = vec![];
        let current_value = match table[self.0 as usize][self.1 as usize] {
            83 => 97, // S -> a
            69 => 122,
            num => num,
        };
        let neighbors: [i32; 3] = [-1, 0, 1];
        for i in neighbors {
            for j in neighbors {
                if i == 0 && j == 0 {
                    continue; // Same as current pos
                }

                if i.abs() == j.abs() {
                    continue; // Cannot move diagonally
                }

                if table.get((self.0 + i) as usize) != None
                    && table
                        .get((self.0 + i) as usize)
                        .unwrap()
                        .get((self.1 + j) as usize)
                        != None
                {
                    let cell_value = table
                        .get((self.0 + i) as usize)
                        .unwrap()
                        .get((self.1 + j) as usize)
                        .unwrap();
                    println!(
                        "cell_value: {} y: {} x: {}",
                        cell_value,
                        self.0 + i,
                        self.1 + j
                    );
                    if cell_value - current_value == 1
                        || cell_value - current_value == 0
                        || (current_value == 122 && *cell_value == 69)
                    {
                        successors.push(PosYX(self.0 + i, self.1 + j))
                    }
                }
            }
        }
        println!(
            "Successors: {:?}\tself:{:?}\t{}",
            &successors, &self, current_value
        );
        successors.into_iter().map(|p| (p, 1)).collect()
    }
}

pub fn output(input: &String) {
    let table: Table = input
        .split('\n')
        .map(|line| line.chars().map(|ch| ch as i32).collect::<Vec<i32>>())
        .collect();
    // S - 83
    // E - 69
    let mut start: PosYX = PosYX::default();
    let mut goal: PosYX = PosYX::default();
    for i in 0..table.len() {
        for j in 0..table[0].len() {
            match table[i][j] {
                83 => start = PosYX(i as i32, j as i32),
                69 => goal = PosYX(i as i32, j as i32),
                _ => {}
            }
        }
    }
    println!("{:#?}", table);
    println!("Start: {:?}\tGoal: {:?}", &start, &goal);
    let custom = PosYX(12, 118);
    println!(
        "Successors of: {:?}\t{:?}",
        &custom,
        custom.successors(&table)
    );
    let res = dijkstra(&start, |p| p.successors(&table), |p| *p == goal).unwrap();

    println!("{:#?}", &res);

    // println!("Shortest path: {}", &res.len())
}
