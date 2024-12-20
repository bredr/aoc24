use regex::Regex;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<(i32, i32)> {
    let re_numbers = Regex::new(r"\d+").unwrap();

    input
        .trim()
        .lines()
        .map(|x| {
            let numbers: Vec<i32> = re_numbers
                .captures_iter(x)
                .map(|y| y[0].parse().unwrap())
                .collect();
            (numbers[0], numbers[1])
        })
        .collect()
}

#[derive(Debug, Eq, PartialEq)]
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
#[derive(Debug)]
pub struct MemorySpace {
    pub corrupted: HashSet<(i32, i32)>,
    pub start: (i32, i32),
    pub end: (i32, i32),
    pub width: i32,
    pub height: i32,
}

impl MemorySpace {
    pub fn find_path(&self) -> u32 {
        let mut distances = HashMap::new();
        let mut queue = BinaryHeap::new();
        let mut visited = HashSet::new();

        // Try all initial directions
        for &direction in &[(1, 0), (0, 1)] {
            queue.push(State {
                cost: 0,
                position: self.start,
                direction,
            });
            distances.insert((self.start, direction), 0);
        }

        while let Some(State {
            cost,
            position,
            direction,
        }) = queue.pop()
        {
            if position == self.end {
                return cost;
            }

            // Skip if we've visited this state with a better cost
            if !visited.insert((position, direction)) {
                continue;
            }

            // Try moving forward
            let next_pos = (position.0 + direction.0, position.1 + direction.1);
            if next_pos.0 >= 0
                && next_pos.0 < self.width
                && next_pos.1 >= 0
                && next_pos.1 < self.height
                && !self.corrupted.contains(&next_pos)
            {
                queue.push(State {
                    cost: cost + 1,
                    position: next_pos,
                    direction,
                });
            }

            // Try turning left and right
            for &new_direction in &[
                rotate90_clockwise(direction),
                rotate90_anticlockwise(direction),
            ] {
                let next_pos = (position.0 + new_direction.0, position.1 + new_direction.1);
                if next_pos.0 >= 0
                    && next_pos.0 < self.width
                    && next_pos.1 >= 0
                    && next_pos.1 < self.height
                    && !self.corrupted.contains(&next_pos)
                {
                    queue.push(State {
                        cost: cost + 1,
                        position: next_pos,
                        direction: new_direction,
                    });
                }
            }
        }

        u32::MAX
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

#[aoc(day18, part1)]
pub fn solve_part1(input: &[(i32, i32)]) -> u32 {
    let space = MemorySpace {
        corrupted: HashSet::from_iter(input[0..1024].iter().map(|x| *x)),
        start: (0, 0),
        end: (70, 70),
        height: 71,
        width: 71,
    };
    space.find_path()
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &[(i32, i32)]) -> String {
    let mut result = String::new();
    for i in 1025..input.len() {
        let space = MemorySpace {
            corrupted: HashSet::from_iter(input[0..i].iter().map(|x| *x)),
            start: (0, 0),
            end: (70, 70),
            height: 71,
            width: 71,
        };
        if space.find_path() == u32::MAX {
            result = format!("{},{}", input[i - 1].0, input[i - 1].1);
            break;
        }
    }
    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let input = "5,4
        4,2
        4,5
        3,0
        2,1
        6,3
        2,4
        1,5
        0,6
        3,3
        2,6
        5,1
        1,2
        5,5
        2,5
        6,5
        1,4
        0,4
        6,4
        1,1
        6,1
        1,0
        0,5
        1,6
        2,0";
        let space = MemorySpace {
            corrupted: HashSet::from_iter(input_generator(input)[0..12].iter().map(|x| *x)),
            start: (0, 0),
            end: (6, 6),
            height: 7,
            width: 7,
        };
        assert_eq!(space.find_path(), 22);
    }
    #[test]
    fn test_part2() {
        let input = "5,4
            4,2
            4,5
            3,0
            2,1
            6,3
            2,4
            1,5
            0,6
            3,3
            2,6
            5,1
            1,2
            5,5
            2,5
            6,5
            1,4
            0,4
            6,4
            1,1
            6,1
            1,0
            0,5
            1,6
            2,0";
        let corruptions = input_generator(input);
        for i in 12..corruptions.len() {
            let space = MemorySpace {
                corrupted: HashSet::from_iter(corruptions[0..i].iter().map(|x| *x)),
                start: (0, 0),
                end: (6, 6),
                height: 7,
                width: 7,
            };
            if space.find_path() == u32::MAX {
                assert_eq!(corruptions[i - 1], (6, 1));
                break;
            }
        }
    }
}
