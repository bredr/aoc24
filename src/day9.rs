use std::collections::HashMap;

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
    let file_sizes: HashMap<usize, u32> = HashMap::from_iter(
        disk_map
            .iter()
            .step_by(2)
            .enumerate()
            .map(|(idx, x)| (idx, *x)),
    );
    let mut disk: Vec<i32> = disk_map
        .iter()
        .enumerate()
        .flat_map(|(idx, x)| {
            (0..*x).map(move |_| {
                let idx = idx.clone();
                match idx % 2 {
                    0 => (idx / 2) as i32,
                    _ => -1,
                }
            })
        })
        .collect();

    let max_file_id = *disk.iter().max().unwrap();

    for file_id in (0..max_file_id + 1).rev() {
        let file_id: usize = file_id.try_into().unwrap();
        if let Some(start_pos) = disk.iter().position(|&x| x == file_id as i32) {
            let file_size = file_sizes.get(&file_id).unwrap();
            let mut best_free_space: Option<usize> = None;
            let mut i = 0;
            while i < start_pos {
                if disk[i] == -1 {
                    let mut free_space_size = 0;
                    let mut pos = i;
                    loop {
                        if pos == disk.len() {
                            break;
                        }
                        if disk[pos] == -1 {
                            free_space_size += 1;
                        } else {
                            break;
                        }
                        pos += 1;
                    }
                    if free_space_size >= *file_size {
                        best_free_space = Some(i);
                        break;
                    }
                    i += free_space_size as usize;
                } else {
                    i += 1;
                }
            }
            if let Some(v) = best_free_space {
                if v < start_pos {
                    // clear old location
                    for i in start_pos..(start_pos + *file_size as usize) {
                        disk[i] = -1;
                    }
                    // place new location
                    for i in v..(v + *file_size as usize) {
                        disk[i] = file_id as i32;
                    }
                }
            }
        }
    }

    disk.iter()
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
