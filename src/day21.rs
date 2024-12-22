use std::collections::HashMap;

use cached::proc_macro::cached;

#[aoc_generator(day21)]
pub fn input_generator(input: &str) -> Vec<(char, char, char)> {
    input
        .trim()
        .lines()
        .map(|x| {
            let mut c = x.trim().chars();
            (c.next().unwrap(), c.next().unwrap(), c.next().unwrap())
        })
        .collect()
}

pub struct Grid {
    buttons: HashMap<char, (isize, isize)>,
}

impl Grid {
    pub fn num_pad_path(&self, from: char, to: char) -> Vec<char> {
        let (ax, ay) = self.buttons.get(&from).unwrap();
        let (bx, by) = self.buttons.get(&to).unwrap();
        let dx = bx - ax;
        let dy = by - ay;
        let horiz: Vec<char> = (0..dx.abs())
            .map(|_| {
                if dx >= 0 {
                    return '>';
                }
                return '<';
            })
            .collect();
        let vert: Vec<char> = (0..dy.abs())
            .map(|_| {
                if dy < 0 {
                    return 'v';
                }
                return '^';
            })
            .collect();

        let mut seq = Vec::<char>::new();
        if *ay == 0 && *bx == 0 {
            seq.extend(vert);
            seq.extend(horiz);
        } else if *ax == 0 && *by == 0 {
            seq.extend(horiz);
            seq.extend(vert);
        } else if dx < 0 {
            seq.extend(horiz);
            seq.extend(vert);
        } else {
            seq.extend(vert);
            seq.extend(horiz);
        }
        seq.push('A');
        seq
    }

    pub fn dir_pad_path(&self, from: char, to: char) -> Vec<char> {
        let (ax, ay) = self.buttons.get(&from).unwrap();
        let (bx, by) = self.buttons.get(&to).unwrap();
        let dx = bx - ax;
        let dy = by - ay;
        let horiz: Vec<char> = (0..dx.abs())
            .map(|_| {
                if dx >= 0 {
                    return '>';
                }
                return '<';
            })
            .collect();
        let vert: Vec<char> = (0..dy.abs())
            .map(|_| {
                if dy < 0 {
                    return 'v';
                }
                return '^';
            })
            .collect();

        let mut seq = Vec::<char>::new();
        if *ax == 0 && *by == 1 {
            seq.extend(horiz);
            seq.extend(vert);
        } else if *ay == 1 && *bx == 0 {
            seq.extend(vert);
            seq.extend(horiz);
        } else if dx < 0 {
            seq.extend(horiz);
            seq.extend(vert);
        } else {
            seq.extend(vert);
            seq.extend(horiz);
        }
        seq.push('A');
        seq
    }
}

#[cached]
pub fn number_pad_instructions(input: Vec<char>, start: char) -> Vec<char> {
    let mut grid = HashMap::<char, (isize, isize)>::new();
    grid.insert('A', (2, 0));
    grid.insert('0', (1, 0));
    grid.insert('1', (0, 1));
    grid.insert('2', (1, 1));
    grid.insert('3', (2, 1));
    grid.insert('4', (0, 2));
    grid.insert('5', (1, 2));
    grid.insert('6', (2, 2));
    grid.insert('7', (0, 3));
    grid.insert('8', (1, 3));
    grid.insert('9', (2, 3));
    let g = Grid { buttons: grid };
    let mut curr = start;
    let mut path = Vec::new();
    for x in input {
        path.extend(g.num_pad_path(curr, x));
        curr = x;
    }
    path
}

#[cached]
pub fn dir_pad_instructions(input: Vec<char>, start: char) -> Vec<char> {
    let mut grid = HashMap::<char, (isize, isize)>::new();
    grid.insert('^', (1, 1));
    grid.insert('v', (1, 0));
    grid.insert('<', (0, 0));
    grid.insert('>', (2, 0));
    grid.insert('A', (2, 1));
    let g = Grid { buttons: grid };
    let mut curr = start;
    let mut path = Vec::new();
    for x in input {
        path.extend(g.dir_pad_path(curr, x));
        curr = x;
    }
    path
}

#[cached]
pub fn count_sequences(input: Vec<char>, num_robots: u32, robot: u32) -> u64 {
    let seq = dir_pad_instructions(input, 'A');
    if robot == num_robots {
        return seq.len().try_into().unwrap();
    }
    let mut count = 0;
    for step in split_sequence(seq) {
        count += count_sequences(step, num_robots, robot + 1);
    }
    count
}

fn split_sequence(input: Vec<char>) -> Vec<Vec<char>> {
    let mut current = Vec::new();
    let mut result = Vec::new();
    for char in input {
        current.push(char);
        if char == 'A' {
            result.push(current.clone());
            current = Vec::new();
        }
    }
    result
}

#[aoc(day21, part1)]
pub fn solve_part1(input: &[(char, char, char)]) -> u64 {
    let mut sum = 0;
    for (a, b, c) in input {
        let moves = number_pad_instructions(vec![*a, *b, *c, 'A'], 'A');
        let count = count_sequences(moves, 2, 1);
        let code: u64 = format!("{}{}{}", a, b, c).parse().unwrap();
        sum += code * count;
    }
    sum
}

#[aoc(day21, part2)]
pub fn solve_part2(input: &[(char, char, char)]) -> u64 {
    let mut sum = 0;
    for (a, b, c) in input {
        let moves = number_pad_instructions(vec![*a, *b, *c, 'A'], 'A');
        let count = count_sequences(moves, 25, 1);
        let code: u64 = format!("{}{}{}", a, b, c).parse().unwrap();
        sum += code * count;
    }
    sum
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let input = "029A
        980A
        179A
        456A
        379A";
        assert_eq!(solve_part1(&input_generator(input)), 126384)
    }
}
