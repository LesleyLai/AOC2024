use itertools::Itertools;
use utils::Grid;

const TEST_INPUT: &str = include_str!("./test_input.txt");
const INPUT: &str = include_str!("./input.txt");

fn parse_input(input: &str) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let mut locks = vec![];
    let mut keys = vec![];

    for schema in input.split("\n\n") {
        let grid = Grid::from_text(schema);
        assert_eq!(grid.height, 7);

        let heights: [u8; 5] = grid
            .columns()
            .map(|column| u8::try_from(column.filter(|&&c| c == b'#').count()).unwrap())
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        if grid.rows().next().unwrap().iter().all(|&c| c == b'#') {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    (locks, keys)
}

fn part1(input: &str) -> usize {
    let (locks, keys) = parse_input(input);

    locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(lock, key)| lock.iter().zip(key.iter()).all(|(a, b)| a + b <= 7))
        .count()
}

fn main() {
    assert_eq!(part1(TEST_INPUT), 3);
    assert_eq!(part1(INPUT), 3327);
}
