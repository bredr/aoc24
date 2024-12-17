use regex::Regex;

pub struct Input {
    pub button_a: (i64, i64),
    pub button_b: (i64, i64),
    pub prize: (i64, i64),
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<Input> {
    input
        .trim()
        .split("\n\n")
        .map(|x| {
            let mut lines = x.lines();
            let re_numbers = Regex::new(r"-?\d+").unwrap();
            let a: Vec<i64> = re_numbers
                .captures_iter(lines.next().unwrap())
                .map(|y| y[0].parse::<i64>().unwrap())
                .collect();
            let b: Vec<i64> = re_numbers
                .captures_iter(lines.next().unwrap())
                .map(|y| y[0].parse::<i64>().unwrap())
                .collect();
            let prize: Vec<i64> = re_numbers
                .captures_iter(lines.next().unwrap())
                .map(|y| y[0].parse::<i64>().unwrap())
                .collect();
            Input {
                button_a: (a[0], a[1]),
                button_b: (b[0], b[1]),
                prize: (prize[0], prize[1]),
            }
        })
        .collect()
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &[Input]) -> u64 {
    let mut total = 0;
    for x in input {
        let (xa, ya) = x.button_a;
        let (xb, yb) = x.button_b;
        let (xprize, yprize) = x.prize;
        let mut lowest_cost: Option<i64> = None;
        for a in 0..100 {
            for b in 0..100 {
                if a * xa + b * xb == xprize && a * ya + b * yb == yprize {
                    if let Some(v) = lowest_cost {
                        if v > a * 3 + b {
                            lowest_cost = Some(a * 3 + b);
                        }
                    } else {
                        lowest_cost = Some(a * 3 + b);
                    }
                }
            }
        }
        if let Some(v) = lowest_cost {
            total += v;
        }
    }
    total.try_into().unwrap()
}

#[aoc(day13, part2)]
pub fn solve_part2(part1_input: &[Input]) -> u64 {
    let input: Vec<Input> = part1_input
        .iter()
        .map(|x| {
            let (px, py) = x.prize;
            Input {
                button_a: x.button_a,
                button_b: x.button_b,
                prize: (10000000000000 + px, 10000000000000 + py),
            }
        })
        .collect();
    let mut total = 0;
    for x in input {
        let (xa, ya) = x.button_a;
        let (xb, yb) = x.button_b;
        let (xp, yp) = x.prize;
        let a_clicks_multiplied = xa * yb - (ya * xb);
        let prize_multiplied = xp * yb - (yp * xb);
        if prize_multiplied % a_clicks_multiplied != 0 {
            continue;
        }
        let a_clicks = prize_multiplied / a_clicks_multiplied;
        if (xp - (xa * a_clicks)) % xb != 0 {
            continue;
        }
        let b_clicks = (xp - (xa * a_clicks)) / xb;
        total += a_clicks * 3 + b_clicks;
    }
    total.try_into().unwrap()
}
