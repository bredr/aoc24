use std::collections::HashMap;

pub struct Input {
    pub towels: Vec<String>,
    pub designs: Vec<String>,
    pub cache: HashMap<String, u64>,
}

#[aoc_generator(day19)]
pub fn input_generator(input: &str) -> Input {
    let mut sections = input.split("\n\n");
    Input {
        towels: sections
            .next()
            .unwrap()
            .trim()
            .split(", ")
            .map(|x| x.trim().to_owned())
            .collect(),
        designs: sections
            .next()
            .unwrap()
            .trim()
            .lines()
            .map(|x| x.trim().to_owned())
            .collect(),
        cache: HashMap::new(),
    }
}

impl Input {
    pub fn solve_design(&self, design: String, towels: Vec<String>) -> bool {
        if design.len() == 0 {
            return true;
        }
        self.towels
            .iter()
            .filter(|x| design.starts_with(*x))
            .any(|x| {
                let mut next_towels = towels.clone();
                next_towels.push(x.clone());
                self.solve_design(design.replacen(x, "", 1), next_towels)
            })
    }

    pub fn number_of_options(&mut self, design: String) -> u64 {
        if design.len() == 0 {
            return 1;
        }
        if let Some(v) = self.cache.get(&design) {
            return *v;
        }
        let result = self
            .towels
            .clone()
            .iter()
            .filter(|x| design.starts_with(*x))
            .map(|x| self.number_of_options(design.replacen(x, "", 1)))
            .sum();
        self.cache.insert(design, result);
        result
    }
}
#[aoc(day19, part1)]
pub fn solve_part1(input: &Input) -> u32 {
    let mut count = 0;
    for design in input.designs.iter() {
        if input.solve_design(design.clone(), Vec::new()) {
            count += 1;
        }
    }
    count
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &Input) -> u64 {
    let mut cached_input = Input {
        designs: input.designs.clone(),
        towels: input.towels.clone(),
        cache: HashMap::new(),
    };
    let mut count = 0;
    for design in input.designs.iter() {
        count += cached_input.number_of_options(design.clone());
    }
    count
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let input = "r, wr, b, g, bwu, rb, gb, br

     brwrr
     bggr
     gbbr
     rrbgbr
     ubwu
     bwurrg
     brgr
     bbrgwb";
        assert_eq!(solve_part1(&input_generator(input)), 6)
    }

    #[test]
    fn test_part2() {
        let input = "r, wr, b, g, bwu, rb, gb, br

     brwrr
     bggr
     gbbr
     rrbgbr
     ubwu
     bwurrg
     brgr
     bbrgwb";
        assert_eq!(solve_part2(&input_generator(input)), 16)
    }
}
