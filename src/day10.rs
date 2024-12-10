use std::collections::HashSet;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Input {
    Input {
        grid: input
            .trim()
            .lines()
            .map(|x| x.trim().chars().map(|y| y.to_digit(10).unwrap()).collect())
            .collect(),
    }
}

pub struct Input {
    pub grid: Vec<Vec<u32>>,
}

impl Input {
    pub fn trailheads(&self) -> Vec<(usize, usize)> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, v)| **v == 0)
                    .map(move |(x, _)| (x, y))
            })
            .collect()
    }

    pub fn trailhead_nines(
        &self,
        start: (i32, i32),
        previous: Option<u32>,
    ) -> HashSet<(usize, usize)> {
        let mut output = HashSet::new();

        let x_max = self.grid[0].len() as i32;
        let y_max = self.grid.len() as i32;
        let (x, y) = start;
        if x < 0 || x >= x_max || y < 0 || y >= y_max {
            return output;
        }
        let v = self.grid[y as usize][x as usize];
        let is_single_step = match previous {
            None => true,
            Some(p) => p + 1 == v,
        };
        if !is_single_step {
            return output;
        }
        if v == 9 {
            output.insert((x as usize, y as usize));
            return output;
        }
        output.extend(self.trailhead_nines((x - 1, y), Some(v)));
        output.extend(self.trailhead_nines((x + 1, y), Some(v)));
        output.extend(self.trailhead_nines((x, y + 1), Some(v)));
        output.extend(self.trailhead_nines((x, y - 1), Some(v)));
        return output;
    }

    pub fn trailhead_score(&self, start: (i32, i32), previous: Option<u32>) -> u32 {
        let x_max = self.grid[0].len() as i32;
        let y_max = self.grid.len() as i32;
        let (x, y) = start;
        if x < 0 || x >= x_max || y < 0 || y >= y_max {
            return 0;
        }
        let v = self.grid[y as usize][x as usize];
        let is_single_step = match previous {
            None => true,
            Some(p) => p + 1 == v,
        };
        if !is_single_step {
            return 0;
        }
        if v == 9 {
            return 1;
        }
        return self.trailhead_score((x - 1, y), Some(v))
            + self.trailhead_score((x + 1, y), Some(v))
            + self.trailhead_score((x, y + 1), Some(v))
            + self.trailhead_score((x, y - 1), Some(v));
    }
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &Input) -> u32 {
    let trailheads = input.trailheads();
    trailheads
        .iter()
        .map(|(x, y)| input.trailhead_nines((*x as i32, *y as i32), None).len() as u32)
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &Input) -> u32 {
    let trailheads = input.trailheads();
    trailheads
        .iter()
        .map(|(x, y)| input.trailhead_score((*x as i32, *y as i32), None))
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let input = Input {
            grid: "89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732"
                .trim()
                .lines()
                .map(|x| x.trim().chars().map(|y| y.to_digit(10).unwrap()).collect())
                .collect(),
        };
        assert_eq!(solve_part1(&input), 36);
    }

    #[test]
    fn test_part2() {
        let input = Input {
            grid: "89010123
        78121874
        87430965
        96549874
        45678903
        32019012
        01329801
        10456732"
                .trim()
                .lines()
                .map(|x| x.trim().chars().map(|y| y.to_digit(10).unwrap()).collect())
                .collect(),
        };
        assert_eq!(solve_part2(&input), 81);
    }
}
