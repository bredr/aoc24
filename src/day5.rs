use std::collections::HashMap;

#[derive(Debug)]
pub struct Input {
    pub rules: Vec<(usize, usize)>,
    pub updates: Vec<Vec<usize>>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let mut iter = input.split("\n\n");
    Input {
        rules: iter
            .next()
            .unwrap()
            .lines()
            .map(|x| x.trim().split("|").map(|y| y.parse().unwrap()).collect())
            .map(|x: Vec<usize>| (x[0], x[1]))
            .collect(),
        updates: iter
            .next()
            .unwrap()
            .lines()
            .map(|x| x.trim().split(",").map(|y| y.parse().unwrap()).collect())
            .collect(),
    }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Input) -> u32 {
    let mut result = 0;
    for update in &input.updates {
        let pages_idx = HashMap::<usize, usize>::from_iter(
            update.iter().enumerate().map(|(idx, page)| (*page, idx)),
        );
        let mut broken_rule = false;
        for (before, after) in &input.rules {
            if pages_idx.contains_key(&before) && pages_idx.contains_key(&after) {
                if pages_idx.get(&before).unwrap() > pages_idx.get(&after).unwrap() {
                    broken_rule = true;
                    break;
                }
            }
        }
        if !broken_rule {
            result += update[update.len() / 2];
        }
    }
    result.try_into().unwrap()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Input) -> u32 {
    let mut result = 0;
    for update in &input.updates {
        let mut pages_idx = HashMap::<usize, usize>::from_iter(
            update.iter().enumerate().map(|(idx, page)| (*page, idx)),
        );
        let mut broken_rule_ever = false;
        let mut broken_rule = true;
        while broken_rule {
            broken_rule = false;
            for (before, after) in &input.rules {
                if pages_idx.contains_key(&before) && pages_idx.contains_key(&after) {
                    if pages_idx.get(&before).unwrap() > pages_idx.get(&after).unwrap() {
                        broken_rule_ever = true;
                        broken_rule = true;
                        let old_before = pages_idx.get(&before).unwrap().clone();
                        let old_after = pages_idx.get(&after).unwrap().clone();
                        pages_idx.insert(*before, old_after);
                        pages_idx.insert(*after, old_before);
                    }
                }
            }
        }
        if broken_rule_ever {
            let updated_update =
                HashMap::<usize, usize>::from_iter(pages_idx.iter().map(|(k, v)| (*v, *k)));

            result += updated_update.get(&(update.len() / 2)).unwrap();
        }
    }
    result.try_into().unwrap()
}
