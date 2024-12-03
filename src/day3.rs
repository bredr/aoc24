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
