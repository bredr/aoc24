use std::collections::{HashMap, HashSet};

#[aoc_generator(day22)]
pub fn input_generator(input: &str) -> Vec<u64> {
    input.lines().map(|x| x.parse().unwrap()).collect()
}

pub fn next_secret_number(y: u64) -> u64 {
    let mut x = y;
    x = (x ^ (x * 64)) % 16777216;
    x = ((x / 32) ^ x) % 16777216;
    x = ((x * 2048) ^ x) % 16777216;
    x
}

pub fn nth_secret_number(x: u64, n: usize) -> u64 {
    let mut secret = x;
    for _ in 0..n {
        secret = next_secret_number(secret);
    }
    secret
}

pub fn nth_prices(x: u64, n: usize) -> HashMap<(isize, isize, isize, isize), u64> {
    let mut prices = Vec::new();
    let mut secret = x;
    for _ in 0..n {
        secret = next_secret_number(secret);
        prices.push(secret % 10);
    }
    let mut result = HashMap::new();
    for i in 4..n {
        let changes = (
            prices[i - 4] as isize - prices[i - 3] as isize,
            prices[i - 3] as isize - prices[i - 2] as isize,
            prices[i - 2] as isize - prices[i - 1] as isize,
            prices[i - 1] as isize - prices[i] as isize,
        );
        result.entry(changes).or_insert(prices[i]);
    }
    result
}

#[aoc(day22, part1)]
pub fn solve_part1(input: &[u64]) -> u64 {
    input.iter().map(|x| nth_secret_number(*x, 2000)).sum()
}

#[aoc(day22, part2)]
pub fn solve_part2(input: &[u64]) -> u64 {
    let buyers = input
        .iter()
        .map(|x| nth_prices(*x, 2000))
        .collect::<Vec<_>>();
    let sequences = HashSet::<(isize, isize, isize, isize)>::from_iter(
        buyers.iter().flat_map(|x| x.iter().map(|(k, _)| *k)),
    );
    sequences
        .iter()
        .map(|seq| {
            buyers
                .iter()
                .map(|buyer| match buyer.get(seq) {
                    Some(v) => *v,
                    None => 0,
                })
                .sum()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(nth_secret_number(123, 1), 15887950)
    }
}
