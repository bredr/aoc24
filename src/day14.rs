use regex::Regex;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<(i32, i32, i32, i32)> {
    let re_numbers = Regex::new(r"-?\d+").unwrap();

    input
        .trim()
        .lines()
        .map(|x| {
            let mut numbers = re_numbers
                .captures_iter(x)
                .map(|y| y[0].parse::<i32>().unwrap());
            (
                numbers.next().unwrap(),
                numbers.next().unwrap(),
                numbers.next().unwrap(),
                numbers.next().unwrap(),
            )
        })
        .collect()
}

pub fn part1(
    input: &[(i32, i32, i32, i32)],
    width: i32,
    height: i32,
    steps: i32,
    vertical_division: i32,
    horizontal_division: i32,
) -> u64 {
    let mut quadrant_a = 0;
    let mut quadrant_b = 0;
    let mut quadrant_c = 0;
    let mut quadrant_d = 0;

    for (x, y, vx, vy) in input {
        let mut next_x = (x + steps * vx) % width;
        let mut next_y = (y + steps * vy) % height;
        if next_x < 0 {
            next_x += width;
        }
        if next_y < 0 {
            next_y += height;
        }
        if next_x == vertical_division {
            continue;
        }
        if next_y == horizontal_division {
            continue;
        }
        if next_x < vertical_division {
            if next_y < horizontal_division {
                quadrant_a += 1;
            } else {
                quadrant_c += 1
            }
        } else {
            if next_y < horizontal_division {
                quadrant_b += 1;
            } else {
                quadrant_d += 1
            }
        }
    }
    quadrant_a * quadrant_b * quadrant_c * quadrant_d
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &[(i32, i32, i32, i32)]) -> u64 {
    part1(input, 101, 103, 100, 50, 51)
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &[(i32, i32, i32, i32)]) -> u64 {
    let mut safety_factors = (0..10000)
        .map(|steps| (steps, part1(input, 101, 103, steps, 50, 51)))
        .collect::<Vec<_>>();

    safety_factors.sort_by_key(|(_, k)| *k);
    safety_factors[0].0.try_into().unwrap()
}
