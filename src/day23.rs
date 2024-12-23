use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[aoc_generator(day23)]
pub fn input_generator(input: &str) -> Vec<(String, String)> {
    input
        .trim()
        .lines()
        .map(|x| {
            let mut s = x.trim().split("-");
            (
                String::from(s.next().unwrap()),
                String::from(s.next().unwrap()),
            )
        })
        .collect()
}

#[aoc(day23, part1)]
pub fn solve_part1(input: &[(String, String)]) -> u64 {
    let mut n = HashMap::<String, HashSet<String>>::new();
    for (a, b) in input {
        if let Some(v) = n.get_mut(a) {
            v.insert(b.clone());
        } else {
            let mut s = HashSet::new();
            s.insert(b.clone());
            n.insert(a.clone(), s);
        }
        if let Some(v) = n.get_mut(b) {
            v.insert(a.clone());
        } else {
            let mut s = HashSet::new();
            s.insert(a.clone());
            n.insert(b.clone(), s);
        }
    }
    let mut triplets = HashSet::<(String, String, String)>::new();
    for (k, v) in n.iter() {
        let kk = k.clone();
        if kk.starts_with("t") {
            for c in Vec::from_iter(v.iter().map(|x| x.clone()))
                .iter()
                .map(|x| x.clone())
                .combinations(2)
            {
                let c1 = c[0].clone();
                let c2 = c[1].clone();
                if n.get(&c2).unwrap().contains(&c1) {
                    let mut t = vec![kk.clone(), c1, c2];
                    t.sort();
                    let mut it = t.iter();
                    triplets.insert((
                        it.next().unwrap().clone(),
                        it.next().unwrap().clone(),
                        it.next().unwrap().clone(),
                    ));
                }
            }
        }
    }
    triplets.len().try_into().unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let input = "kh-tc
        qp-kh
        de-cg
        ka-co
        yn-aq
        qp-ub
        cg-tb
        vc-aq
        tb-ka
        wh-tc
        yn-cg
        kh-ub
        ta-co
        de-co
        tc-td
        tb-wq
        wh-td
        ta-ka
        td-qp
        aq-cg
        wq-ub
        ub-vc
        de-ta
        wq-aq
        wq-vc
        wh-yn
        ka-de
        kh-ta
        co-tc
        wh-qp
        tb-vc
        td-yn";
        assert_eq!(solve_part1(&input_generator(input)), 7)
    }
}
