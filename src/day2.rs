#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|x| x.trim().split(' ').map(|y| y.parse().unwrap()).collect())
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Vec<i32>]) -> u32 {
    input
        .iter()
        .filter(|x| is_safe(*x))
        .count()
        .try_into()
        .unwrap()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Vec<i32>]) -> u32 {
    input
        .iter()
        .filter(|x| damper(x).iter().any(|y| is_safe(y)))
        .count()
        .try_into()
        .unwrap()
}

fn damper(input: &Vec<i32>) -> Vec<Vec<i32>> {
    let mut options = vec![input.clone()];
    for n in 0..(input.len()) {
        let mut x = input.clone();
        x.remove(n);
        options.push(x);
    }
    options
}

fn is_safe(report: &Vec<i32>) -> bool {
    let is_decreasing = report[0] > report[1];
    report[1..].iter().enumerate().all(|(idx, x)| {
        ((report[idx] > *x) == is_decreasing)
            && (report[idx] - *x).abs() <= 3
            && (report[idx] - *x).abs() >= 1
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe() {
        let result = vec![7, 6, 4, 2, 1];
        assert_eq!(is_safe(&result), true);
    }

    #[test]
    fn test_damper() {
        let input = vec![0, 1, 2];
        assert_eq!(
            damper(&input),
            vec![vec![0, 1, 2], vec![1, 2], vec![0, 2], vec![0, 1]]
        )
    }
}
