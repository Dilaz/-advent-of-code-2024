#[path = "../../utils.rs"]
pub mod utils;

use std::collections::BTreeSet;

pub use utils::Solution;
use miette::Result;
pub struct Day9;

#[derive(Debug, PartialEq, Clone, Eq, Copy)]
pub enum Block {
    Free,
    Used(u32),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DataBlock {
    r#type: Block,
    count: u32,
    index: usize,
}

impl PartialOrd for DataBlock {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for DataBlock {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}

impl DataBlock {
    pub fn new(block: Block, count: u32, index: usize) -> Self {
        Self {
            r#type: block,
            count,
            index,
        }
    }
}

pub fn parse_part1(input: &str) -> Vec<Block> {
    let mut free_space = false;
    let mut blocks = Vec::with_capacity(100_000);
    let mut block_id = 0;
    input.chars().for_each(|c| {
        if free_space {
            for _ in 0..c.to_digit(10).unwrap() {
                blocks.push(Block::Free);
            }
        } else {
            for _ in 0..c.to_digit(10).unwrap() {
                blocks.push(Block::Used(block_id));
            }
            block_id += 1;
        }

        free_space = !free_space;
    });

    blocks
}

pub fn parse_part2(input: &str) -> Vec<DataBlock> {
    let mut free_space = false;
    let mut blocks = Vec::with_capacity(100_000);
    let mut block_id = 0;
    let mut real_index = 0_usize;
    input.chars().for_each(|c|{
        let count = c.to_digit(10).unwrap();
        if count != 0 {
            if free_space {
                blocks.push(DataBlock {
                    r#type: Block::Free,
                    count,
                    index: real_index,
                });
            } else {
                blocks.push(DataBlock {
                    r#type: Block::Used(block_id),
                    count,
                    index: real_index,
                });
                block_id += 1;
            }
            real_index += count as usize;
        }

        free_space = !free_space;
    });

    blocks
}

impl Solution<u64> for Day9 {
    #[tracing::instrument]
    fn part1(input: &str) -> Result<u64> {
        let mut checksum: u64 = 0;
        let blocks = parse_part1(input);
        let iter = blocks.iter().enumerate();
        let mut reverse_iter = iter.clone().rev();
        let mut rev_pos: Option<usize> = None;
        for (pos, block) in iter {
            if rev_pos.is_some() && rev_pos.unwrap() <= pos {
                break;
            }
            match block {
                Block::Used(id) => {
                    checksum += pos as u64 * *id as u64;
                },
                Block::Free => {
                    // Use while here to allow stopping at any point
                    #[allow(clippy::while_let_on_iterator)]
                    while let Some((new_rev_pos, block)) = reverse_iter.next() {
                        if new_rev_pos <= pos {
                            break;
                        }
                        match block {
                            Block::Used(id) => {
                                rev_pos = Some(new_rev_pos);
                                checksum += pos as u64 * *id as u64;
                                break;
                            },
                            Block::Free => {},
                        }
                    }
                }
            }
        }

        Ok(checksum)
    }

    #[tracing::instrument]
    fn part2(input: &str) -> Result<u64> {
        let blocks = parse_part2(input);
        let mut moved_blocks = BTreeSet::new();
        let mut spaces = blocks.iter().filter(|b| b.r#type == Block::Free).cloned().collect::<BTreeSet<_>>();

        for block in blocks.iter().rev() {
            let mut to_add: Option<DataBlock> = None;
            let mut to_remove: Option<DataBlock> = None;
            match block.r#type {
                Block::Used(_) => {
                    if let Some(empty_block) = spaces.iter().find(|b| b.r#type == Block::Free && b.index < block.index && b.count >= block.count) {
                        let diff = empty_block.count - block.count;
                        moved_blocks.insert(DataBlock::new(block.r#type, block.count, empty_block.index));
                        moved_blocks.insert(DataBlock::new(Block::Free, block.count, block.index));

                        if diff > 0 {
                            to_add = Some(DataBlock::new(Block::Free, diff, empty_block.index + block.count as usize));
                        }

                        to_remove = Some(empty_block.clone());
                    } else {
                        // No position found, just move the block to the end
                        moved_blocks.insert(DataBlock::new(block.r#type, block.count, block.index));
                    }
                },
                Block::Free => {
                    moved_blocks.insert(DataBlock::new(block.r#type, block.count, block.index));
                },
            }

            if let Some(add) = to_add {
                spaces.insert(add);
            }
            if let Some(remove) = to_remove {
                spaces.remove(&remove);
            }
        }

        Ok(moved_blocks.into_iter().map(|b| {
            (b.index..b.index + b.count as usize).map(|i| i as u64).sum::<u64>() * match b.r#type {
                Block::Used(id) => id as u64,
                Block::Free => 0,
            }
        }).sum())
    }
    
}


#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_parse_1() {
        let test = r#"12345"#;
        let result = parse_part1(test);
        assert_eq!(result, vec![
            Block::Used(0),
            Block::Free,
            Block::Free,
            Block::Used(1),
            Block::Used(1),
            Block::Used(1),
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Free,
            Block::Used(2),
            Block::Used(2),
            Block::Used(2),
            Block::Used(2),
            Block::Used(2),
        ]);
    }

    #[test]
    fn test_parse_2() {
        let test = r#"12345"#;
        let result = parse_part2(test);
        assert_eq!(result, vec![
            DataBlock { r#type: Block::Used(0), count: 1, index: 0 },
            DataBlock { r#type: Block::Free, count: 2, index: 1 },
            DataBlock { r#type: Block::Used(1), count: 3, index: 3 },
            DataBlock { r#type: Block::Free, count: 4, index: 6 },
            DataBlock { r#type: Block::Used(2), count: 5, index: 10 },
        ]);
    }

    #[test]
    fn test_part1() {
        let test = r#"2333133121414131402"#;
        let result = Day9::part1(test.trim());
        assert_eq!(result.unwrap(), 1928);
    }

    #[test]
    fn test_part2() {
        let test = r#"2333133121414131402"#;
        let result = Day9::part2(test.trim());
        assert_eq!(result.unwrap(), 2858)
    }
}
