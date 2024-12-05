use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let map = reader
            .lines()
            .map(|s| s.unwrap().into_bytes())
            .collect::<Vec<_>>();
        let mut visited = HashSet::new();
        fn to_the_right(diff: [i32; 2]) -> [i32; 2] {
            match diff {
                [0, -1] => [1, 0],
                [1, 0] => [0, 1],
                [0, 1] => [-1, 0],
                [-1, 0] => [0, -1],
                _ => unreachable!("{diff:?}"),
            }
        }
        let mut direction: [i32; 2] = [0, -1];
        let (mut position, _) = map
            .iter()
            .enumerate()
            .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, v)| ([x, y], v)))
            .find(|(_, &v)| v == b'^')
            .unwrap();

        visited.insert(position);
        while let Some((updated_pos, &e)) = position[1]
            .checked_add_signed(direction[1].try_into().unwrap())
            .and_then(|y| map.get(y).map(|r| (y, r)))
            .and_then(|(y, r)| {
                position[0]
                    .checked_add_signed(direction[0].try_into().unwrap())
                    .map(|x| ([x, y], r))
            })
            .and_then(|([x, y], r)| r.get(x).map(|e| ([x, y], e)))
        {
            if e == b'#' {
                direction = to_the_right(direction);
                continue;
            }
            visited.insert(updated_pos);
            position = updated_pos;
        }

        let answer = visited.len();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let map = reader
            .lines()
            .map(|s| s.unwrap().into_bytes())
            .collect::<Vec<_>>();
        let mut tried_blockading = HashSet::new();
        fn to_the_right(diff: [i32; 2]) -> [i32; 2] {
            match diff {
                [0, -1] => [1, 0],
                [1, 0] => [0, 1],
                [0, 1] => [-1, 0],
                [-1, 0] => [0, -1],
                _ => unreachable!("{diff:?}"),
            }
        }
        let start_direction: [i32; 2] = [0, -1];
        let (start_position, _) = map
            .iter()
            .enumerate()
            .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, v)| ([x, y], v)))
            .find(|(_, &v)| v == b'^')
            .unwrap();
        let mut position = start_position;
        let mut direction = start_direction;

        tried_blockading.insert(position);
        let mut loop_count: usize = 0;
        while let Some((updated_pos, &e)) = position[1]
            .checked_add_signed(direction[1].try_into().unwrap())
            .and_then(|y| map.get(y).map(|r| (y, r)))
            .and_then(|(y, r)| {
                position[0]
                    .checked_add_signed(direction[0].try_into().unwrap())
                    .map(|x| ([x, y], r))
            })
            .and_then(|([x, y], r)| r.get(x).map(|e| ([x, y], e)))
        {
            //print!("[{},{}], ", updated_pos[0], updated_pos[1]);
            if e == b'#' {
                direction = to_the_right(direction);
                continue;
            }
            if tried_blockading.insert(updated_pos) {
                let mut phantom_pos = start_position;
                let mut phantom_dir = start_direction;
                let mut been_here = HashSet::new();
                while let Some((phantom_updated, &e)) = phantom_pos[1]
                    .checked_add_signed(phantom_dir[1].try_into().unwrap())
                    .and_then(|y| map.get(y).map(|r| (y, r)))
                    .and_then(|(y, r)| {
                        phantom_pos[0]
                            .checked_add_signed(phantom_dir[0].try_into().unwrap())
                            .map(|x| ([x, y], r))
                    })
                    .and_then(|([x, y], r)| r.get(x).map(|e| ([x, y], e)))
                {
                    if !been_here.insert((phantom_updated, phantom_dir)) {
                        loop_count += 1;
                        break;
                    }
                    if e == b'#' || phantom_updated == updated_pos {
                        phantom_dir = to_the_right(phantom_dir);
                        continue;
                    }
                    phantom_pos = phantom_updated;
                }
            }
            position = updated_pos;
        }

        let answer = loop_count;
        Ok(answer)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
