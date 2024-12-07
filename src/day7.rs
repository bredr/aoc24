pub struct Calculation {
    pub numbers: Vec<i64>,
    pub result: i64,
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Calculation> {
    input
        .trim()
        .lines()
        .map(|x| {
            let mut parts = x.trim().split(":");
            let result_part = parts.next().unwrap();
            let numbers_part = parts.next().unwrap();
            Calculation {
                result: result_part.trim().parse().unwrap(),
                numbers: numbers_part
                    .trim()
                    .split(" ")
                    .map(|y| y.parse().unwrap())
                    .collect(),
            }
        })
        .collect()
}

impl Calculation {
    pub fn is_result_part1(&self, operators: Vec<char>) -> bool {
        if operators.len() == 0 {
            return self.is_result_part1(vec!['+']) || self.is_result_part1(vec!['*']);
        }
        let mut total = self.numbers[0];
        self.numbers[1..(operators.len() + 1)]
            .iter()
            .zip(operators.clone())
            .for_each(|(n, o)| {
                total = match o {
                    '*' => total * n,
                    '+' => total + n,
                    _ => total,
                };
            });
        if total > self.result {
            return false;
        }
        if operators.len() + 1 < self.numbers.len() {
            let mut add = operators.clone();
            add.push('+');
            let mut multiply = operators.clone();
            multiply.push('*');
            return self.is_result_part1(add) || self.is_result_part1(multiply);
        }
        return total == self.result;
    }

    pub fn is_result_part2(&self, operators: Vec<char>) -> bool {
        if operators.len() == 0 {
            return self.is_result_part2(vec!['+'])
                || self.is_result_part2(vec!['*'])
                || self.is_result_part2(vec!['|']);
        }
        let mut total = self.numbers[0];
        self.numbers[1..(operators.len() + 1)]
            .iter()
            .zip(operators.clone())
            .for_each(|(n, o)| {
                total = match o {
                    '*' => total * n,
                    '+' => total + n,
                    '|' => format!("{}{}", total, n).parse().unwrap(),
                    _ => total,
                };
            });
        if total > self.result {
            return false;
        }
        if operators.len() + 1 < self.numbers.len() {
            let mut add = operators.clone();
            add.push('+');
            let mut multiply = operators.clone();
            multiply.push('*');
            let mut concat = operators.clone();
            concat.push('|');
            return self.is_result_part2(add)
                || self.is_result_part2(multiply)
                || self.is_result_part2(concat);
        }
        return total == self.result;
    }
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &[Calculation]) -> u64 {
    input
        .iter()
        .filter(|c| c.is_result_part1(vec![]))
        .map(|c| c.result)
        .sum::<i64>()
        .try_into()
        .unwrap()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &[Calculation]) -> u64 {
    input
        .iter()
        .filter(|c| c.is_result_part2(vec![]))
        .map(|c| c.result)
        .sum::<i64>()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_result_1() {
        let c = Calculation {
            result: 190,
            numbers: vec![10, 19],
        };
        assert_eq!(c.is_result_part1(vec![]), true);
    }
}
