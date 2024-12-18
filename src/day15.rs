use std::collections::HashSet;

pub struct Input {
    pub start: (usize, usize),
    pub boxes: HashSet<(usize, usize)>,
    pub walls: HashSet<(usize, usize)>,
    pub instructions: Vec<char>,
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Input {
    let mut sections = input.split("\n\n");
    let grid = sections
        .next()
        .unwrap()
        .trim()
        .lines()
        .map(|x| x.trim().chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut start = (0, 0);
    let mut boxes = HashSet::<(usize, usize)>::new();
    let mut walls = HashSet::<(usize, usize)>::new();

    grid.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, v)| {
            if *v == '@' {
                start = (x, y);
            }
            if *v == '#' {
                walls.insert((x, y));
            }
            if *v == 'O' {
                boxes.insert((x, y));
            }
        })
    });

    Input {
        start,
        boxes,
        walls,
        instructions: sections
            .next()
            .unwrap()
            .trim()
            .replace("\n", "")
            .chars()
            .collect(),
    }
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &Input) -> u64 {
    let mut robot_position = input.start.clone();
    let mut boxes = input.boxes.clone();
    for instruction in input.instructions.clone() {
        let (x, y) = robot_position;
        let mut next = robot_position.clone();
        let mut v: (i32, i32) = (0, 0);
        if instruction == '<' {
            next.0 = x - 1;
            v.0 = -1;
        } else if instruction == '>' {
            next.0 = x + 1;
            v.0 = 1;
        } else if instruction == '^' {
            next.1 = y - 1;
            v.1 = -1;
        } else if instruction == 'v' {
            next.1 = y + 1;
            v.1 = 1;
        } else {
            continue;
        }
        if input.walls.contains(&next) {
            continue;
        }
        let mut boxes_to_move = Vec::<(usize, usize, usize, usize)>::new();
        let mut next_box = (next.0, next.1);
        let mut hit_wall = false;
        loop {
            if boxes.contains(&next_box) {
                let current_box = next_box.clone();
                next_box = (
                    (next_box.0 as i32 + v.0) as usize,
                    (next_box.1 as i32 + v.1) as usize,
                );
                if input.walls.contains(&next_box) {
                    boxes_to_move = vec![];
                    hit_wall = true;
                    break;
                } else if boxes.contains(&next_box) {
                    boxes_to_move.push((current_box.0, current_box.1, next_box.0, next_box.1));
                } else {
                    boxes_to_move.push((current_box.0, current_box.1, next_box.0, next_box.1));
                    break;
                }
            } else {
                break;
            }
        }
        boxes_to_move.reverse();
        for (old_x, old_y, new_x, new_y) in boxes_to_move {
            boxes.remove(&(old_x, old_y));
            boxes.insert((new_x, new_y));
        }
        if !hit_wall {
            robot_position = next;
        }
    }
    boxes.iter().map(|(x, y)| (x + y * 100) as u64).sum::<u64>()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1_simple() {
        let input = "########
        #..O.O.#
        ##@.O..#
        #...O..#
        #.#.O..#
        #...O..#
        #......#
        ########

        <^^>>>vv<v>>v<<";
        assert_eq!(solve_part1(&input_generator(input)), 2028);
    }
}
