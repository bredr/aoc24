#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<(i32, i32)> {
    input
        .lines()
        .map(|l| {
            let mut row = l
                .trim()
                .split(' ')
                .filter(|x| x.len() > 0)
                .map(|d| d.parse().unwrap());
            (row.next().unwrap(), row.next().unwrap())
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[(i32, i32)]) -> u32 {
    let mut left: Vec<i32> = input.iter().map(|(x, _)| *x).collect();
    left.sort_by(|a, b| a.cmp(b));
    let mut right: Vec<i32> = input.iter().map(|(_, x)| *x).collect();
    right.sort_by(|a, b| a.cmp(b));
    let result = left
        .iter()
        .enumerate()
        .map(|(idx, x)| (x - right[idx]).abs())
        .sum::<i32>();
    result.try_into().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[(i32, i32)]) -> u32 {
    let left: Vec<i32> = input.iter().map(|(x, _)| *x).collect();
    let right: Vec<i32> = input.iter().map(|(_, x)| *x).collect();

    left.iter()
        .map(|x| (right.iter().filter(|y| x == *y).collect::<Vec<_>>().len() as i32) * x)
        .sum::<i32>()
        .try_into()
        .unwrap()
}
