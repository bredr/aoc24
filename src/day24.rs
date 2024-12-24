use std::collections::HashMap;

pub struct Input {
    pub init_wires: HashMap<String, bool>,
    pub connections: HashMap<String, (String, String, String)>,
}

#[aoc_generator(day24)]
pub fn input_generator(input: &str) -> Input {
    let mut sections = input.trim().split("\n\n");
    let init_wires = HashMap::from_iter(sections.next().unwrap().lines().map(|x| {
        let mut y = x.trim().split(": ");
        let wire = y.next().unwrap();
        let value = match y.next().unwrap().trim() {
            "1" => true,
            "0" => false,
            _ => panic!("unexpected"),
        };
        (wire.to_string(), value)
    }));
    let connections = HashMap::from_iter(sections.next().unwrap().lines().map(|x| {
        let mut parts = x.trim().split(" -> ");
        let mut operation = parts.next().unwrap().split(" ");
        let a = operation.next().unwrap().to_string();
        let raw_op = operation.next().unwrap().to_string();
        let b = operation.next().unwrap().to_string();

        let output = parts.next().unwrap();
        (output.to_string(), (a, raw_op, b))
    }));
    Input {
        init_wires,
        connections,
    }
}

#[aoc(day24, part1)]
pub fn solve_part1(input: &Input) -> u64 {
    let mut connections = input.connections.clone();
    let mut wires = input.init_wires.clone();
    loop {
        for (output, (a, op, b)) in connections.clone().iter() {
            if wires.contains_key(a) && wires.contains_key(b) {
                let a_value = *wires.get(a).unwrap();
                let b_value = *wires.get(b).unwrap();
                let value;
                if op == "XOR" {
                    value = a_value ^ b_value;
                } else if op == "OR" {
                    value = a_value || b_value;
                } else {
                    value = a_value && b_value;
                }
                wires.insert(output.clone(), value);
                connections.remove(output);
                break;
            }
        }
        if connections.is_empty() {
            break;
        }
    }
    let mut z_wires: Vec<(String, bool)> = wires
        .iter()
        .filter(|(k, _)| k.starts_with("z"))
        .map(|(k, v)| (k.clone(), *v))
        .collect();
    z_wires.sort_by_key(|a| a.0.clone());
    z_wires
        .iter()
        .enumerate()
        .map(|(i, (_, v))| match v {
            true => u64::pow(2, i as u32),
            false => 0,
        })
        .sum::<u64>()
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1() {
        let input = "x00: 1
        x01: 0
        x02: 1
        x03: 1
        x04: 0
        y00: 1
        y01: 1
        y02: 1
        y03: 1
        y04: 1

        ntg XOR fgs -> mjb
        y02 OR x01 -> tnw
        kwq OR kpj -> z05
        x00 OR x03 -> fst
        tgd XOR rvg -> z01
        vdt OR tnw -> bfw
        bfw AND frj -> z10
        ffh OR nrd -> bqk
        y00 AND y03 -> djm
        y03 OR y00 -> psh
        bqk OR frj -> z08
        tnw OR fst -> frj
        gnj AND tgd -> z11
        bfw XOR mjb -> z00
        x03 OR x00 -> vdt
        gnj AND wpb -> z02
        x04 AND y00 -> kjc
        djm OR pbm -> qhw
        nrd AND vdt -> hwm
        kjc AND fst -> rvg
        y04 OR y02 -> fgs
        y01 AND x02 -> pbm
        ntg OR kjc -> kwq
        psh XOR fgs -> tgd
        qhw XOR tgd -> z09
        pbm OR djm -> kpj
        x03 XOR y03 -> ffh
        x00 XOR y04 -> ntg
        bfw OR bqk -> z06
        nrd XOR fgs -> wpb
        frj XOR qhw -> z04
        bqk OR frj -> z07
        y03 OR x01 -> nrd
        hwm AND bqk -> z03
        tgd XOR rvg -> z12
        tnw OR pbm -> gnj";

        assert_eq!(solve_part1(&input_generator(input)), 2024);
    }
}
