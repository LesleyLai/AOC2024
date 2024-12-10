use std::collections::BTreeMap;
use std::ops::Bound::{Excluded, Included};

const TEST_INPUT: &str = "2333133121414131402";

const INPUT: &str = include_str!("./input.txt");

fn part1(input: &str) -> usize {
    let mut blocks: Vec<Option<usize>> = Vec::new();

    let mut is_free = false;
    let mut id = 0;
    for c in input.as_bytes() {
        let length: u8 = *c - b'0';
        if is_free {
            for _ in 0..length {
                blocks.push(None);
            }

            id = id + 1;
        } else {
            for _ in 0..length {
                blocks.push(Some(id));
            }
        }
        is_free = !is_free;
    }

    for i in 0..blocks.len() {
        if blocks[i] == None {
            for j in ((i + 1)..blocks.len()).rev() {
                if blocks[j] != None {
                    blocks.swap(i, j);
                    break;
                }
            }
        }
    }

    let mut checksum = 0;
    for (i, x) in blocks.iter().enumerate() {
        match x {
            Some(x) => checksum += x * i,
            None => {
                break;
            }
        }
    }
    checksum
}

struct Chunk {
    length: u8,
    id: Option<u16>, // None for free block, and Some for a file
}

impl Chunk {
    fn new_file(length: u8, id: u16) -> Chunk {
        Chunk {
            length,
            id: Some(id),
        }
    }

    fn new_free_space(length: u8) -> Chunk {
        Chunk { length, id: None }
    }
}

fn compute_checksum(chunks: &BTreeMap<usize, Chunk>) -> usize {
    let mut checksum = 0;
    for (start, block) in chunks.iter() {
        if let Some(id) = block.id {
            for i in 0..block.length {
                let offset = start + (i as usize);
                checksum += offset * (id as usize);
            }
        }
    }
    checksum
}

fn read_chunks(input: &str) -> BTreeMap<usize, Chunk> {
    let mut chunks = BTreeMap::new();

    let mut current: usize = 0;
    for (i, &c) in input.as_bytes().iter().enumerate() {
        let length = c - b'0';
        if i % 2 != 0 {
            chunks.insert(current, Chunk::new_free_space(length));
        } else {
            chunks.insert(
                current,
                Chunk::new_file(length, u16::try_from(i / 2).unwrap()),
            );
        };
        current += length as usize;
    }

    chunks
}

fn part2(input: &str) -> usize {
    let mut chunks = read_chunks(input);

    let mut last_start = usize::MAX;

    let mut last_id = u16::MAX;
    loop {
        // Find the location of the next file to move

        let to_move_pair = chunks
            // optimization to shrink the range of search
            .range((Included(&0), Excluded(&last_start)))
            .rev()
            .find(|(_, chunk)| chunk.id.is_some_and(|id| id < last_id));

        // Nothing to move, we are done!
        if to_move_pair.is_none() {
            break;
        }

        let (&to_move, to_move_chunk) = to_move_pair.unwrap();
        let id = to_move_chunk.id.unwrap();
        let length = to_move_chunk.length;
        last_id = id;
        last_start = to_move;

        // find free slot
        let mut free_slot = None;
        for (free_start, free_block) in chunks.iter() {
            // Can't move to later place
            if *free_start >= to_move {
                break;
            }

            if free_block.id.is_none() && free_block.length >= length {
                free_slot = Some(*free_start);
                break;
            }
        }

        // move
        if let Some(free_start) = free_slot {
            *chunks.get_mut(&to_move).unwrap() = Chunk::new_free_space(length);

            let free_slot_size = chunks.remove(&free_start).unwrap().length;
            assert!(free_slot_size >= length);
            let remaining = free_slot_size - length;

            chunks.insert(free_start, Chunk::new_file(length, id));
            if remaining > 0 {
                let remaining_start = free_start + (length as usize);
                chunks.insert(remaining_start, Chunk::new_free_space(remaining));
            }
        }
    }

    compute_checksum(&chunks)
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 1928);
    assert_eq!(part1(INPUT), 6337367222422);

    assert_eq!(part2(TEST_INPUT), 2858);
    assert_eq!(part2(INPUT), 6361380647183);
}
