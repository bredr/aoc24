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
