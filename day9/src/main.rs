use std::collections::BTreeMap;

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

    //print_blocks(&blocks);

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

// fn print_spaces(spaces: &BTreeMap<usize, (u8, Option<usize>)>) {
//     for (len, id) in spaces.values() {
//         match *id {
//             None => {
//                 for _ in 0..*len {
//                     print!(".");
//                 }
//             }
//             Some(id) => {
//                 assert!(id < 10);
//                 for _ in 0..*len {
//                     print!("{}", id);
//                 }
//             }
//         }
//     }
//     println!();
// }

fn compute_checksum(spaces: &BTreeMap<usize, (u8, Option<usize>)>) -> usize {
    let mut checksum = 0;
    for (start, (length, id)) in spaces.iter() {
        if let Some(id) = id {
            for i in 0..*length {
                let offset = start + (i as usize);
                checksum += offset * id;
            }
        }
    }
    checksum
}

fn part2(input: &str) -> usize {
    let mut spaces = BTreeMap::new();

    let mut is_free = false;
    let mut id: usize = 0;
    let mut current: usize = 0;
    for c in input.as_bytes() {
        let length: u8 = *c - b'0';
        let id = if is_free {
            id = id + 1;
            None
        } else {
            Some(id)
        };

        spaces.insert(current, (length, id));

        current += length as usize;
        is_free = !is_free;
    }

    //print_spaces(&spaces);

    let mut last_id = usize::MAX;
    loop {
        // Find the location of the next file to move
        let mut to_move = None;
        for (start, (_, id)) in spaces.iter().rev() {
            if let Some(id) = id {
                if *id < last_id {
                    to_move = Some(start);
                    last_id = *id;
                    break;
                }
            }
        }
        // Nothing to move, we are done!
        if to_move.is_none() {
            break;
        }

        let to_move = *to_move.unwrap();
        let (length, id) = spaces.get(&to_move).unwrap();
        let length = *length;
        let id = id.unwrap();

        // find free slot

        let mut free_slot = None;
        for (free_start, (free_length, free_id)) in spaces.iter() {
            // Can't move to later place
            if *free_start >= to_move {
                break;
            }

            if free_id.is_none() {
                if *free_length >= length {
                    free_slot = Some(*free_start);
                    break;
                }
            }
        }

        // move
        if let Some(free_start) = free_slot {
            *spaces.get_mut(&to_move).unwrap() = (length, None);

            let (free_slot_size, _) = spaces.remove(&free_start).unwrap();
            assert!(free_slot_size >= length);
            let remaining = free_slot_size - length;

            spaces.insert(free_start, (length, Some(id)));
            if remaining > 0 {
                spaces.insert(free_start + (length as usize), (remaining, None));
            }
        }
    }

    compute_checksum(&spaces)
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 1928);
    assert_eq!(part1(INPUT), 6337367222422);

    assert_eq!(part2(TEST_INPUT), 2858);
    assert_eq!(part2(INPUT), 6361380647183);
}
