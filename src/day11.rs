use cached::proc_macro::cached;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input
        .trim()
        .split(" ")
        .map(|x| x.parse().unwrap())
        .collect()
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &[u64]) -> u64 {
    let mut stones: Vec<u64> = input.to_vec();
    for _ in 0..25 {
        stones = mutate_stones(stones);
    }
    stones.len().try_into().unwrap()
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &[u64]) -> u64 {
    input.iter().map(|x| count_mutate_stones(*x, 75)).sum()
}

pub fn mutate_stones(input: Vec<u64>) -> Vec<u64> {
    input.iter().flat_map(|x| mutate_stone(*x)).collect()
}

#[cached]
pub fn count_mutate_stones(input: u64, depth: usize) -> u64 {
    let stones = mutate_stone(input);
    if depth == 1 {
        return stones.len() as u64;
    }
    return stones
        .iter()
        .map(|x| count_mutate_stones(*x, depth - 1))
        .sum();
}

#[cached]
pub fn mutate_stone(input: u64) -> Vec<u64> {
    if input == 0 {
        return vec![1];
    }
    let input_str = format!("{}", input);
    if input_str.len() % 2 == 0 {
        let (left, right) = input_str.split_at(input_str.len() / 2);
        return vec![left.parse().unwrap(), right.parse().unwrap()];
    }
    return vec![input * 2024];
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let input = [125, 17];
        assert_eq!(solve_part1(&input), 55312)
    }
}
