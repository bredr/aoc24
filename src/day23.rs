use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

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

pub fn group(k: String, n: HashMap<String, HashSet<String>>) -> HashSet<String> {
    let mut result = HashSet::new();
    let mut last_total = 0;
    let mut queue = Vec::new();
    queue.push(&k);
    while let Some(v) = queue.pop() {
        let new = n.get(v).unwrap();
        result.extend(new.clone());
        queue.extend(new);
        if result.len() == last_total {
            break;
        }
        last_total = result.len();
    }
    result
}

#[aoc(day23, part2)]
pub fn solve_part2(input: &[(String, String)]) -> String {
    let mut computers = HashSet::<String>::new();
    let mut connections = HashSet::<(String, String)>::new();
    for (a, b) in input {
        computers.insert(a.to_owned());
        computers.insert(b.to_owned());
        connections.insert((a.to_owned(), b.to_owned()));
        connections.insert((b.to_owned(), a.to_owned()));
    }

    let mut seen_groups = Vec::<Vec<String>>::new();

    let mut computers: Vec<String> = computers.into_iter().collect();
    computers.sort_unstable();
    let mut computers: VecDeque<String> = computers.into_iter().collect();

    while let Some(computer) = computers.pop_front() {
        let mut curr_group = vec![computer];
        'outer: loop {
            for (idx, other_computer) in computers.iter().enumerate() {
                if curr_group.iter().all(|curr_computer| {
                    connections.contains(&(curr_computer.to_owned(), other_computer.to_owned()))
                }) {
                    curr_group.push(computers.remove(idx).unwrap());
                    continue 'outer;
                }
            }
            break;
        }
        seen_groups.push(curr_group);
    }
    seen_groups.sort_by_key(|x| x.len());

    let mut x = Vec::from_iter(seen_groups.last().unwrap().iter().map(|x| x.clone()));
    x.sort();
    return x.join(",");
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
