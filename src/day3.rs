use regex::Regex;

#[aoc(day3, part1)]
pub fn solve_part1(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").unwrap();
    let mut result = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|x| x.extract()) {
        result += i32::from_str_radix(a, 10).unwrap() * i32::from_str_radix(b, 10).unwrap();
    }
    result.try_into().unwrap()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3})\,(\d{1,3})\)").unwrap();
    let dont_re = Regex::new(r"don\'t\(\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let mut result = 0;

    let subs = dont_re
        .split(input)
        .enumerate()
        .map(|(idx, x)| {
            if idx == 0 {
                return x;
            }
            let dos = do_re.splitn(x, 2).collect::<Vec<&str>>();
            if dos.len() > 1 {
                return dos[1];
            }
            ""
        })
        .collect::<Vec<_>>();
    for sub in subs {
        for (_, [a, b]) in re.captures_iter(sub).map(|x| x.extract()) {
            result += i32::from_str_radix(a, 10).unwrap() * i32::from_str_radix(b, 10).unwrap();
        }
    }
    result.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(solve_part2(&input), 48);
    }
}
