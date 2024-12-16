use std::collections::{HashMap, HashSet};

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .lines()
        .map(|x| x.trim().chars().collect())
        .collect()
}

pub fn find_regions(
    starting_point: (usize, usize),
    mut this_visited: HashSet<(i32, i32)>,
    mut this_border: HashSet<(i32, i32, i32, i32)>,
    grid: &Vec<Vec<char>>,
) -> (HashSet<(i32, i32)>, HashSet<(i32, i32, i32, i32)>) {
    let (x, y) = starting_point;
    let plant_type = grid[y][x];
    this_visited.insert((x as i32, y as i32));
    if (x as i32) - 1 >= 0 && grid[y][x - 1] == plant_type {
        if !this_visited.contains(&((x - 1) as i32, y as i32)) {
            let (visited, border) =
                find_regions((x - 1, y), this_visited.clone(), this_border.clone(), grid);
            this_visited.extend(visited);
            this_border.extend(border);
        }
    } else {
        this_border.insert((x as i32, y as i32, x as i32 - 1, y as i32));
    }
    if x + 1 < grid[0].len() && grid[y][x + 1] == plant_type {
        if !this_visited.contains(&((x + 1) as i32, y as i32)) {
            let (visited, border) =
                find_regions((x + 1, y), this_visited.clone(), this_border.clone(), grid);
            this_visited.extend(visited);
            this_border.extend(border);
        }
    } else {
        this_border.insert((x as i32, y as i32, (x + 1) as i32, y as i32));
    }
    if (y as i32) - 1 >= 0 && grid[y - 1][x] == plant_type {
        if !this_visited.contains(&(x as i32, (y - 1) as i32)) {
            let (visited, border) =
                find_regions((x, y - 1), this_visited.clone(), this_border.clone(), grid);
            this_visited.extend(visited);
            this_border.extend(border);
        }
    } else {
        this_border.insert((x as i32, y as i32, x as i32, y as i32 - 1));
    }

    if y + 1 < grid.len() && grid[y + 1][x] == plant_type {
        if !this_visited.contains(&(x as i32, (y + 1) as i32)) {
            let (visited, border) =
                find_regions((x, y + 1), this_visited.clone(), this_border.clone(), grid);
            this_visited.extend(visited);
            this_border.extend(border);
        }
    } else {
        this_border.insert((x as i32, y as i32, x as i32, (y + 1) as i32));
    }

    (this_visited, this_border)
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> u64 {
    let mut all_visited = HashSet::<(i32, i32)>::new();
    let mut total_cost: u64 = 0;
    for x in 0..input[0].len() {
        for y in 0..input.len() {
            if !all_visited.contains(&(x as i32, y as i32)) {
                let (visited, border) =
                    find_regions((x, y), HashSet::new(), HashSet::new(), &input.to_vec());
                total_cost += (visited.len() * border.len()) as u64;
                all_visited.extend(visited);
            }
        }
    }
    total_cost
}

fn border_to_sides(border: HashSet<(i32, i32, i32, i32)>) -> u64 {
    let mut sides = HashMap::<(i32, i32, char), Vec<i32>>::new();
    for (x_in, y_in, x_out, y_out) in border {
        if x_in == x_out {
            //vertical fence
            if let Some(v) = sides.get_mut(&(y_in, y_out, 'v')) {
                v.push(x_out);
            } else {
                let mut x = Vec::new();
                x.push(x_out);
                sides.insert((y_in, y_out, 'v'), x);
            }
        } else {
            //horizontal fence
            if let Some(v) = sides.get_mut(&(x_in, x_out, 'h')) {
                v.push(y_out);
            } else {
                let mut x = Vec::new();
                x.push(y_out);
                sides.insert((x_in, x_out, 'h'), x);
            }
        }
    }
    let mut count = 0;
    for (_, v) in sides {
        let mut indices = Vec::from_iter(v);
        indices.sort();
        let mut last = indices[0];
        for i in indices {
            if i == last {
                count += 1;
            } else if last + 1 != i {
                count += 1;
            }
            last = i;
        }
    }
    count
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> u64 {
    let mut all_visited = HashSet::<(i32, i32)>::new();
    let mut total_cost: u64 = 0;
    for x in 0..input[0].len() {
        for y in 0..input.len() {
            if !all_visited.contains(&(x as i32, y as i32)) {
                let (visited, border) =
                    find_regions((x, y), HashSet::new(), HashSet::new(), &input.to_vec());
                let sides = border_to_sides(border);
                total_cost += visited.len() as u64 * sides;
                all_visited.extend(visited);
            }
        }
    }
    total_cost
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1_simple() {
        let input = input_generator(
            "AAAA
        BBCD
        BBCC
        EEEC",
        );

        assert_eq!(solve_part1(&input), 140)
    }

    #[test]
    fn test_part1_internal() {
        let input = input_generator(
            "OOOOO
            OXOXO
            OOOOO
            OXOXO
            OOOOO",
        );

        assert_eq!(solve_part1(&input), 772)
    }

    #[test]
    fn test_part2_internal() {
        let input = input_generator(
            "EEEEE
            EXXXX
            EEEEE
            EXXXX
            EEEEE",
        );

        assert_eq!(solve_part2(&input), 236)
    }
}
