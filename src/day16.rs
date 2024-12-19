use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

pub struct Input {
    pub start: (i32, i32),
    pub end: (i32, i32),
    pub walls: HashSet<(i32, i32)>,
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
    let mut walls = HashSet::<(i32, i32)>::new();
    let mut start: (i32, i32) = (0, 0);
    let mut end: (i32, i32) = (0, 0);
    input.trim().lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, v)| {
            if v == '#' {
                walls.insert((x as i32, y as i32));
            }
            if v == 'S' {
                start = (x as i32, y as i32);
            }
            if v == 'E' {
                end = (x as i32, y as i32);
            }
        });
    });
    Input { walls, start, end }
}

#[derive(Eq, PartialEq)]
pub struct State {
    cost: u32,
    position: (i32, i32),
    direction: (i32, i32),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Input {
    pub fn find_path(&self) -> u32 {
        let mut distances = HashMap::new();
        let mut queue = BinaryHeap::new();

        // Initialize with starting state
        queue.push(State {
            cost: 0,
            position: self.start,
            direction: (1, 0),
        });
        distances.insert((self.start, (0, 1)), 0);

        while let Some(State {
            cost,
            position,
            direction,
        }) = queue.pop()
        {
            // If this state has been seen with a better cost, skip it
            if let Some(&dist) = distances.get(&(position, direction)) {
                if cost > dist {
                    continue;
                }
            }

            // Found the end
            if position == self.end {
                return cost;
            }

            // Skip if we hit a wall
            if self.walls.contains(&position) {
                continue;
            }

            // Continue in same direction
            let next_pos = (position.0 + direction.0, position.1 + direction.1);
            let next_cost = cost + 1;
            if !self.walls.contains(&next_pos) {
                if let Some(&dist) = distances.get(&(next_pos, direction)) {
                    if next_cost < dist {
                        distances.insert((next_pos, direction), next_cost);
                        queue.push(State {
                            cost: next_cost,
                            position: next_pos,
                            direction,
                        });
                    }
                } else {
                    distances.insert((next_pos, direction), next_cost);
                    queue.push(State {
                        cost: next_cost,
                        position: next_pos,
                        direction,
                    });
                }
            }

            // Handle turns
            for &new_direction in &[
                rotate90_clockwise(direction),
                rotate90_anticlockwise(direction),
            ] {
                let turn_cost = cost + 1000;
                if let Some(&dist) = distances.get(&(position, new_direction)) {
                    if turn_cost < dist {
                        distances.insert((position, new_direction), turn_cost);
                        queue.push(State {
                            cost: turn_cost,
                            position,
                            direction: new_direction,
                        });
                    }
                } else {
                    distances.insert((position, new_direction), turn_cost);
                    queue.push(State {
                        cost: turn_cost,
                        position,
                        direction: new_direction,
                    });
                }
            }
        }

        u32::MAX // No path found
    }
}
pub fn rotate90_clockwise(d: (i32, i32)) -> (i32, i32) {
    match d {
        (0, 1) => (-1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (1, 0),
        (-1, 0) => (0, 1),
        (x, y) => (x, y),
    }
}
pub fn rotate90_anticlockwise(d: (i32, i32)) -> (i32, i32) {
    match d {
        (0, 1) => (1, 0),
        (1, 0) => (0, 1),
        (0, -1) => (-1, 0),
        (-1, 0) => (0, -1),
        (x, y) => (x, y),
    }
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &Input) -> u32 {
    input.find_path()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1_simple() {
        let input = "###############
        #.......#....E#
        #.#.###.#.###.#
        #.....#.#...#.#
        #.###.#####.#.#
        #.#.#.......#.#
        #.#.#####.###.#
        #...........#.#
        ###.#.#####.#.#
        #...#.....#.#.#
        #.#.#.###.#.#.#
        #.....#...#.#.#
        #.###.#.#.#.#.#
        #S..#.....#...#
        ###############";
        assert_eq!(solve_part1(&input_generator(input)), 7036);
    }
}
