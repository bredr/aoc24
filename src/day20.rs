use std::collections::HashSet;

pub struct Input {
    pub start: (usize, usize),
    pub end: (usize, usize),
    pub walls: HashSet<(usize, usize)>,
}

#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> Input {
    let mut walls = HashSet::<(usize, usize)>::new();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    input.trim().lines().enumerate().for_each(|(y, row)| {
        row.chars().enumerate().for_each(|(x, v)| {
            if v == '#' {
                walls.insert((x, y));
            }
            if v == 'S' {
                start = (x, y);
            }
            if v == 'E' {
                end = (x, y);
            }
        });
    });
    Input { walls, start, end }
}

impl Input {
    pub fn course(&self) -> Vec<(usize, usize)> {
        self._course(vec![self.start])
    }

    fn _course(&self, route: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let (x, y) = route[route.len() - 1];
        if (x, y) == self.end {
            return route;
        }
        for option in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
            if !self.walls.contains(&option) {
                let route_set =
                    HashSet::<(usize, usize)>::from_iter(route.clone().iter().map(|x| *x));
                if !route_set.contains(&option) {
                    let mut next_route = route.clone();
                    next_route.push(option);
                    return self._course(next_route);
                }
            }
        }
        panic!("");
    }

    pub fn every_cheat(&self) -> u64 {
        let mut count = 0;
        let course = self.course();
        for (idx, point) in course.clone()[0..course.len() - 101].iter().enumerate() {
            for other_point in course[idx + 101..].iter() {
                let d = distance(*point, *other_point);
                if d <= 2 {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn every_cheat2(&self) -> u64 {
        let mut count = 0;
        let course = self.course();
        for (idx, point) in course.clone()[0..course.len() - 101].iter().enumerate() {
            for (id2, other_point) in course[idx + 101..].iter().enumerate() {
                let original_distance = id2 + 101;
                let d = distance(*point, *other_point);
                if d <= 20 && original_distance - d >= 100 {
                    count += 1;
                }
            }
        }
        count
    }
}
fn absolute_difference(x: usize, y: usize) -> usize {
    if x > y {
        x - y
    } else {
        y - x
    }
}
pub fn distance((ax, ay): (usize, usize), (bx, by): (usize, usize)) -> usize {
    absolute_difference(ax, bx) + absolute_difference(ay, by)
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &Input) -> u64 {
    input.every_cheat()
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &Input) -> u64 {
    input.every_cheat2()
}
