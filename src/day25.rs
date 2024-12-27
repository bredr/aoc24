#[derive(Debug)]
pub struct Input {
    pub locks: Vec<Vec<u32>>,
    pub keys: Vec<Vec<u32>>,
}

#[aoc_generator(day25)]
pub fn input_generator(input: &str) -> Input {
    let mut keys = Vec::new();
    let mut locks = Vec::new();
    for block in input.split("\n\n") {
        let block_grid: Vec<Vec<char>> = block
            .trim()
            .lines()
            .map(|x| x.trim().chars().collect())
            .collect();
        let is_key = block_grid[0].iter().all(|x| *x == '#');
        let block_counts: Vec<u32> = (0..5)
            .map(|x| block_grid.iter().filter(|y| y[x] == '#').count() as u32 - 1)
            .collect();
        if is_key {
            keys.push(block_counts);
        } else {
            locks.push(block_counts);
        }
    }
    Input { locks, keys }
}

#[aoc(day25, part1)]
pub fn solve_part1(input: &Input) -> u32 {
    let mut count = 0;
    for key in input.keys.clone() {
        for lock in input.locks.clone() {
            if key.iter().zip(lock.iter()).all(|(&x, &y)| x + y < 6) {
                count += 1;
            }
        }
    }
    count
}
