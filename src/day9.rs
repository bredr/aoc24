#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> u64 {
    let disk_map: Vec<u32> = input.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let disk_len: u32 = disk_map.iter().sum();
    let mut disk = Vec::<i32>::with_capacity(disk_len as usize);
    let mut is_block = true;
    let mut id = 0;
    for entry in disk_map.clone() {
        if is_block {
            for _ in 0..entry {
                disk.push(id);
            }
            id += 1;
        } else {
            for _ in 0..entry {
                disk.push(-1);
            }
        }
        is_block = !is_block;
    }

    let mut empty_index: Vec<usize> = disk
        .iter()
        .enumerate()
        .filter(|(_, v)| **v == -1)
        .map(|(idx, _)| idx)
        .collect();
    empty_index.reverse();
    let mut block_index: Vec<usize> = disk
        .iter()
        .enumerate()
        .filter(|(_, v)| **v != -1)
        .map(|(idx, _)| idx)
        .collect();
    let full_len = block_index.len();
    loop {
        let next_block = block_index.pop().unwrap();
        let next_empty = empty_index.pop().unwrap();
        if next_empty >= full_len {
            break;
        }
        disk.swap(next_block, next_empty);
    }
    disk.iter()
        .filter(|x| **x != -1)
        .enumerate()
        .map(|(idx, id)| idx as u64 * *id as u64)
        .sum()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> u64 {
    let disk_map: Vec<u32> = input.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut block_sizes: Vec<(usize, u32)> = disk_map
        .iter()
        .step_by(2)
        .enumerate()
        .map(|(idx, x)| (idx, *x))
        .collect();
    let empty_sizes: Vec<(usize, u32)> = disk_map
        .iter()
        .skip(1)
        .step_by(2)
        .enumerate()
        .map(|(idx, x)| (idx, *x))
        .collect();
    let mut disk: Vec<Vec<i32>> = disk_map
        .iter()
        .enumerate()
        .map(|(idx, x)| {
            (0..*x)
                .map(move |_| {
                    let idx = idx.clone();
                    match idx % 2 {
                        0 => (idx / 2) as i32,
                        _ => -1,
                    }
                })
                .collect()
        })
        .collect();

    block_sizes.reverse();
    for (empty_idx, empty_size) in empty_sizes {
        let mut stack = Vec::<(usize, u32)>::new();
        for (idx, size) in block_sizes.clone() {
            if stack.iter().map(|(_, s)| *s as u32).sum::<u32>() + size <= empty_size {
                stack.push((idx, size));
                disk[2 * idx] = (0..size).map(|_| -1).collect();
            }
            if stack.iter().map(|(_, s)| *s as u32).sum::<u32>() == empty_size {
                break;
            }
        }
        block_sizes = block_sizes
            .iter()
            .filter(|(id, _)| !stack.iter().any(|(oid, _)| oid == id))
            .map(|x| *x)
            .collect();

        let mut stack_block: Vec<i32> = stack
            .iter()
            .flat_map(|(idx, size)| (0..*size).map(|_| *idx as i32))
            .collect();
        if stack_block.len() < empty_size as usize {
            for _ in 0..(empty_size - stack_block.len() as u32) {
                stack_block.push(-1);
            }
        }
        disk[empty_idx * 2 + 1] = stack_block;
    }

    disk.iter()
        .flatten()
        .enumerate()
        .map(|(idx, id)| match *id {
            -1 => 0,
            v => idx as u64 * v as u64,
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_result_2() {
        assert_eq!(solve_part2("2333133121414131402"), 2858);
    }
}
