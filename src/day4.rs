use regex::Regex;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|x| x.trim().chars().collect()).collect()
}

fn match_xmas(s: &str) -> usize {
    let releft = Regex::new(r"XMAS").unwrap();
    let reright = Regex::new(r"SAMX").unwrap();

    releft.captures_iter(s).count() + reright.captures_iter(s).count()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[Vec<char>]) -> u32 {
    let mut total = 0;
    for row in input {
        let row_str = join_chars(row.clone());
        total += match_xmas(row_str.as_str());
    }
    for column_idx in 0..input[0].len() {
        let column = input.iter().map(|x| x[column_idx]).collect::<Vec<char>>();
        let column_str = join_chars(column.clone());
        total += match_xmas(column_str.as_str());
    }

    let x_max = input[0].len() as i32;
    let y_max = input.len() as i32;

    for x in -1 * x_max..2 * x_max {
        let mut d1 = Vec::new();
        let mut d2 = Vec::new();
        for i in 0..y_max {
            if x + i >= 0 && x + i < x_max {
                d1.push(input[i as usize][(x + i) as usize]);
            }
            if x - i >= 0 && x - i < x_max {
                d2.push(input[i as usize][(x - i) as usize]);
            }
        }
        let d1_str = join_chars(d1.clone());
        total += match_xmas(d1_str.as_str());
        let d2_str = join_chars(d2.clone());
        total += match_xmas(d2_str.as_str());
    }

    total.try_into().unwrap()
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[Vec<char>]) -> u32 {
    let x_max = input[0].len() as i32;
    let y_max = input.len() as i32;
    let mut count = 0;
    for x in 1..(x_max - 1) {
        for y in 1..(y_max - 1) {
            let y: usize = y as usize;
            let x: usize = x as usize;
            if input[y][x] == 'A' {
                if input[y - 1][x - 1] != input[y + 1][x + 1]
                    && input[y + 1][x - 1] != input[y - 1][x + 1]
                {
                    if (input[y - 1][x - 1] == 'S' || input[y - 1][x - 1] == 'M')
                        && (input[y - 1][x + 1] == 'S' || input[y - 1][x + 1] == 'M')
                        && (input[y + 1][x - 1] == 'S' || input[y + 1][x - 1] == 'M')
                        && (input[y + 1][x + 1] == 'S' || input[y + 1][x + 1] == 'M')
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn join_chars(x: Vec<char>) -> String {
    let mut s = String::new();
    for y in x {
        s.push(y);
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input: Vec<Vec<char>> = "MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX"
            .lines()
            .map(|x| x.trim().chars().collect())
            .collect();
        assert_eq!(solve_part1(&input), 18);
    }

    #[test]
    fn test_part_2() {
        let input: Vec<Vec<char>> = "MMMSXXMASM
            MSAMXMSMSA
            AMXSXMAAMM
            MSAMASMSMX
            XMASAMXAMM
            XXAMMXXAMA
            SMSMSASXSS
            SAXAMASAAA
            MAMMMXMMMM
            MXMXAXMASX"
            .lines()
            .map(|x| x.trim().chars().collect())
            .collect();
        assert_eq!(solve_part2(&input), 9);
    }
}
