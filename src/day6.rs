use std::collections::HashSet;

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|x| x.trim().chars().collect()).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> u32 {
    let mut visited = Vec::<(usize, usize)>::new();
    let mut obstacles = HashSet::<(usize, usize)>::new();
    let y_max = input.len() as i32;
    let x_max = input[0].len() as i32;
    for (y, row) in input.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            if *v == '#' {
                obstacles.insert((x, y));
            }
            if *v == '^' {
                visited.push((x, y));
            }
        }
    }
    let mut direction: (i32, i32) = (0, -1);
    loop {
        let (x, y) = visited[visited.len() - 1];
        let (xx, yy) = direction;
        let next_x = x as i32 + xx;
        let next_y = y as i32 + yy;
        if next_x < 0 || next_x >= x_max || next_y < 0 || next_y >= y_max {
            break;
        }
        let next: (usize, usize) = (next_x.try_into().unwrap(), next_y.try_into().unwrap());
        if obstacles.contains(&next) {
            direction = match direction {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                (_, _) => direction,
            };
        } else {
            visited.push(next);
        }
    }
    HashSet::<(usize, usize)>::from_iter(visited.iter().map(|x| *x))
        .len()
        .try_into()
        .unwrap()
}

struct Grid {
    obstacles: HashSet<(usize, usize)>,
    x_max: i32,
    y_max: i32,
}

impl Grid {
    pub fn is_loop(&self, start: (usize, usize, i32, i32), new_obstacle: (usize, usize)) -> bool {
        let mut obstacles = self.obstacles.clone();
        obstacles.insert(new_obstacle);
        let mut visited = HashSet::<(usize, usize, i32, i32)>::new();
        let mut here = start.clone();
        visited.insert(here);
        loop {
            let (x, y, xx, yy) = here;
            let next_x = x as i32 + xx;
            let next_y = y as i32 + yy;
            if next_x < 0 || next_x >= self.x_max || next_y < 0 || next_y >= self.y_max {
                return false;
            }
            let next: (usize, usize) = (next_x.try_into().unwrap(), next_y.try_into().unwrap());
            if obstacles.contains(&next) {
                here = match here {
                    (x, y, 0, -1) => (x, y, 1, 0),
                    (x, y, 1, 0) => (x, y, 0, 1),
                    (x, y, 0, 1) => (x, y, -1, 0),
                    (x, y, -1, 0) => (x, y, 0, -1),
                    (_, _, _, _) => here,
                };
                if visited.contains(&here) {
                    return true;
                }
                visited.insert(here);
            } else {
                let (x, y) = next;
                here = (x, y, xx, yy);
                if visited.contains(&here) {
                    return true;
                }
                visited.insert(here);
            }
        }
    }
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> u32 {
    let mut visited = Vec::<(usize, usize)>::new();
    let mut obstacles = HashSet::<(usize, usize)>::new();
    let y_max = input.len() as i32;
    let x_max = input[0].len() as i32;
    for (y, row) in input.iter().enumerate() {
        for (x, v) in row.iter().enumerate() {
            if *v == '#' {
                obstacles.insert((x, y));
            }
            if *v == '^' {
                visited.push((x, y));
            }
        }
    }
    let mut direction: (i32, i32) = (0, -1);
    loop {
        let (x, y) = visited[visited.len() - 1];
        let (xx, yy) = direction;
        let next_x = x as i32 + xx;
        let next_y = y as i32 + yy;
        if next_x < 0 || next_x >= x_max || next_y < 0 || next_y >= y_max {
            break;
        }
        let next: (usize, usize) = (next_x.try_into().unwrap(), next_y.try_into().unwrap());
        if obstacles.contains(&next) {
            direction = match direction {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                (_, _) => direction,
            };
        } else {
            visited.push(next);
        }
    }
    let (x, y) = visited[0];
    let start = (x, y, 0, -1);
    let grid = Grid {
        x_max,
        y_max,
        obstacles,
    };

    let mut count = 0;
    for new_obstacle in HashSet::<(usize, usize)>::from_iter(visited[1..].iter().map(|x| *x)) {
        if grid.is_loop(start.clone(), new_obstacle) {
            count += 1;
        }
    }
    count
}
