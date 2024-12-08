use std::collections::{HashMap, HashSet};

pub struct Input {
    pub antennae: HashMap<char, Vec<(usize, usize)>>,
    pub x_max: i32,
    pub y_max: i32,
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Input {
    let mut antennae = HashMap::<char, Vec<(usize, usize)>>::new();
    input.trim().lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, char)| {
            if char != '.' {
                if antennae.contains_key(&char) {
                    antennae.get_mut(&char).unwrap().push((x, y));
                } else {
                    antennae.insert(char, vec![(x, y)]);
                }
            }
        })
    });
    Input {
        antennae,
        y_max: input.trim().lines().count() as i32,
        x_max: input.trim().lines().next().unwrap().trim().chars().count() as i32,
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &Input) -> u64 {
    let mut anti_nodes = HashSet::<(usize, usize)>::new();
    for (_, nodes) in input.antennae.clone() {
        for a in nodes.clone() {
            for b in nodes.clone() {
                if a != b {
                    let (xa, ya) = a;
                    let (xb, yb) = b;

                    let x1: i32 = 2 * xa as i32 - xb as i32;
                    let x2: i32 = 2 * xb as i32 - xa as i32;
                    let y1: i32 = 2 * ya as i32 - yb as i32;
                    let y2: i32 = 2 * yb as i32 - ya as i32;
                    if x1 >= 0 && x1 < input.x_max && y1 >= 0 && y1 < input.y_max {
                        anti_nodes.insert((x1.try_into().unwrap(), y1.try_into().unwrap()));
                    }
                    if x2 >= 0 && x2 < input.x_max && y2 >= 0 && y2 < input.y_max {
                        anti_nodes.insert((x2.try_into().unwrap(), y2.try_into().unwrap()));
                    }
                }
            }
        }
    }
    anti_nodes.len().try_into().unwrap()
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Input) -> u64 {
    let mut anti_nodes = HashSet::<(usize, usize)>::new();
    for (_, nodes) in input.antennae.clone() {
        for a in nodes.clone() {
            for b in nodes.clone() {
                if a != b {
                    let (xa, ya) = a;
                    let (xb, yb) = b;
                    let mut n: i32 = 1;
                    anti_nodes.insert(a);
                    anti_nodes.insert(b);
                    loop {
                        let x1: i32 = xa as i32 - n * (xb as i32 - xa as i32);
                        let x2: i32 = xb as i32 + n * (xb as i32 - xa as i32);
                        let y1: i32 = ya as i32 - n * (yb as i32 - ya as i32);
                        let y2: i32 = yb as i32 + n * (yb as i32 - ya as i32);
                        let mut added = false;
                        if x1 >= 0 && x1 < input.x_max && y1 >= 0 && y1 < input.y_max {
                            anti_nodes.insert((x1.try_into().unwrap(), y1.try_into().unwrap()));
                            added = true;
                        }
                        if x2 >= 0 && x2 < input.x_max && y2 >= 0 && y2 < input.y_max {
                            anti_nodes.insert((x2.try_into().unwrap(), y2.try_into().unwrap()));
                            added = true;
                        }
                        if !added {
                            break;
                        }
                        n += 1;
                    }
                }
            }
        }
    }
    anti_nodes.len().try_into().unwrap()
}
