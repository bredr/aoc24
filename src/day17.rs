use regex::Regex;

#[derive(Debug)]
pub struct Input {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub instructions: Vec<u32>,
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> Input {
    let re_numbers = Regex::new(r"-?\d+").unwrap();

    let mut lines = input.lines();

    let a: u32 = re_numbers
        .captures_iter(lines.next().unwrap())
        .next()
        .unwrap()[0]
        .parse()
        .unwrap();
    let b: u32 = re_numbers
        .captures_iter(lines.next().unwrap())
        .next()
        .unwrap()[0]
        .parse()
        .unwrap();
    let c: u32 = re_numbers
        .captures_iter(lines.next().unwrap())
        .next()
        .unwrap()[0]
        .parse()
        .unwrap();
    lines.next();
    let instructions: Vec<u32> = re_numbers
        .captures_iter(lines.next().unwrap())
        .map(|x| x[0].parse().unwrap())
        .collect();
    Input {
        a,
        b,
        c,
        instructions,
    }
}

fn combo(operand: u32, a: u32, b: u32, c: u32) -> u32 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => a,
        5 => b,
        6 => c,
        _ => panic!("reserved"),
    }
}

impl Input {
    pub fn exec(&mut self, mut instruction_pointer: usize, mut outputs: Vec<u32>) -> String {
        if instruction_pointer >= self.instructions.len() {
            return outputs
                .iter()
                .map(|x| format!("{}", x))
                .collect::<Vec<_>>()
                .join::<&str>(",");
        }
        let opcode = self.instructions[instruction_pointer];
        let operand = self.instructions[instruction_pointer + 1];
        let mut jumped = false;

        if opcode == 0 {
            let base: u32 = 2;
            self.a = self.a / base.pow(combo(operand, self.a, self.b, self.c));
        } else if opcode == 1 {
            self.b = self.b ^ operand;
        } else if opcode == 2 {
            self.b = combo(operand, self.a, self.b, self.c) % 8;
        } else if opcode == 3 {
            if self.a != 0 {
                jumped = true;
                instruction_pointer = operand as usize;
            }
        } else if opcode == 4 {
            self.b = self.b ^ self.c;
        } else if opcode == 5 {
            outputs.push(combo(operand, self.a, self.b, self.c) % 8);
        } else if opcode == 6 {
            let base: u32 = 2;
            self.b = self.a / base.pow(combo(operand, self.a, self.b, self.c));
        } else if opcode == 7 {
            let base: u32 = 2;
            self.c = self.a / base.pow(combo(operand, self.a, self.b, self.c));
        }
        if !jumped {
            instruction_pointer += 2;
        }
        return self.exec(instruction_pointer, outputs);
    }
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &Input) -> String {
    let mut computer = Input {
        a: input.a,
        b: input.b,
        c: input.c,
        instructions: input.instructions.clone(),
    };
    computer.exec(0, Vec::new())
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &Input) -> u32 {
    let mut a = 0;
    let target = input
        .instructions
        .clone()
        .iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<_>>()
        .join::<&str>(",");
    loop {
        let mut computer = Input {
            a,
            b: input.b,
            c: input.c,
            instructions: input.instructions.clone(),
        };
        if computer.exec(0, Vec::new()) == target {
            break;
        }

        a += 1;
    }
    a
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part1_case1() {
        let input = "Register A: 729
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0";
        assert_eq!(solve_part1(&input_generator(input)), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part1_case2() {
        let input = "Register A: 10
        Register B: 0
        Register C: 0

        Program: 5,0,5,1,5,4";
        assert_eq!(solve_part1(&input_generator(input)), "0,1,2");
    }

    #[test]
    fn test_part1_case3() {
        let input = "Register A: 2024
        Register B: 0
        Register C: 0

        Program: 0,1,5,4,3,0";
        assert_eq!(
            solve_part1(&input_generator(input)),
            "4,2,5,6,7,7,7,7,3,1,0"
        );
    }
    #[test]
    fn test_part1_case4() {
        let mut computer = Input {
            a: 0,
            b: 29,
            c: 0,
            instructions: vec![1, 7],
        };
        computer.exec(0, Vec::new());
        assert_eq!(computer.b, 26)
    }

    #[test]
    fn test_part1_case5() {
        let mut computer = Input {
            a: 0,
            b: 2024,
            c: 43690,
            instructions: vec![4, 0],
        };
        computer.exec(0, Vec::new());
        assert_eq!(computer.b, 44354)
    }

    #[test]
    fn test_part1_case6() {
        let mut computer = Input {
            a: 0,
            b: 0,
            c: 9,
            instructions: vec![2, 6],
        };
        computer.exec(0, Vec::new());
        assert_eq!(computer.b, 1)
    }
}
