use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::BitAnd;

const DAY: &str = "09"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
2333133121414131402
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut file = Vec::new();
        let bytes = reader.read_to_end(&mut file).unwrap();
        if bytes.bitand(1) != 1 {
            file.pop();
        }

        for b in file.iter_mut() {
            *b -= b'0'
        }

        let mut front = file.iter().copied().enumerate();
        let mut back = file.iter().copied().enumerate().rev().step_by(2); // We don't care about back space
        let mut left_of_back_file = 0;
        let mut back_file_id = usize::MAX;
        let mut curr_block = 0;
        let mut curr_check_sum = 0;
        'outer: while let Some((i, front_file)) = front.next() {
            let file_id = i / 2;
            if file_id == back_file_id {
                break;
            }
            for _ in 0..front_file {
                curr_check_sum += file_id * curr_block;
                curr_block += 1;
            }
            let space = front.next().unwrap().1;
            for _ in 0..space {
                if left_of_back_file == 0 {
                    let (i, back_file) = back.next().unwrap();
                    back_file_id = i / 2;
                    if back_file_id == file_id {
                        break 'outer;
                    }
                    left_of_back_file = back_file;
                }
                curr_check_sum += back_file_id * curr_block;
                curr_block += 1;
                left_of_back_file -= 1;
            }
        }
        for _ in 0..left_of_back_file {
            curr_check_sum += back_file_id * curr_block;
            curr_block += 1;
        }

        return Ok(curr_check_sum);
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut file = Vec::new();
        let bytes = reader.read_to_end(&mut file).unwrap();
        if bytes % 2 == 0 {
            file.pop();
        }

        for b in file.iter_mut() {
            *b -= b'0'
        }

        let mut expanded_file = Vec::new();
        for (i, e) in file.iter().copied().enumerate() {
            for _ in 0..e {
                expanded_file.push(if i % 2 == 0 { i / 2 } else { usize::MAX })
            }
        }

        let mut curr_pos = expanded_file.len() - 1;

        'outer: loop {
            while expanded_file[curr_pos] == usize::MAX {
                curr_pos -= 1;
            }

            let id = expanded_file[curr_pos];
            let mut count = 0;
            while expanded_file[curr_pos] == id {
                if curr_pos == 0 {
                    break 'outer;
                }
                count += 1;
                curr_pos -= 1;
            }

            let mut curr_streak: usize = 0;
            let mut curr_streak_start = 0;
            for (i, v) in expanded_file
                .iter()
                .copied()
                .enumerate()
                .take_while(|(i, _)| *i <= curr_pos)
            {
                if v != usize::MAX {
                    curr_streak = 0;
                    curr_streak_start = i + 1;
                    continue;
                }
                curr_streak += 1;
                if curr_streak == count {
                    break;
                }
            }
            if curr_streak < count {
                continue;
            }
            for i in (0..count).map(|i| i + curr_streak_start) {
                assert_eq!(expanded_file[i], usize::MAX);
                expanded_file[i] = id;
            }
            for i in (0..count).map(|i| i + curr_pos + 1) {
                assert_eq!(expanded_file[i], id);
                expanded_file[i] = usize::MAX
            }
        }
        Ok(expanded_file
            .into_iter()
            .enumerate()
            .filter(|(_, id)| *id != usize::MAX)
            .map(|(i, id)| i * id)
            .sum())
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
