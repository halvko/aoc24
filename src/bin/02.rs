use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle
        let mut input = bad_input::BadInput::new(reader);
        let answer = input
            .lines()
            .map(|l| l.split(" ").map(|e| e.parse::<u32>()).collect::<Vec<_>>())
            .filter(|v| {
                v.windows(2).all(|w| {
                    let lhs = w[0];
                    let rhs = w[1];
                    lhs.abs_diff(rhs) >= 1 && lhs.abs_diff(rhs) <= 3
                }) && (v.iter().is_sorted_by(|lhs, rhs| lhs <= rhs)
                    || v.iter().is_sorted_by(|lhs, rhs| lhs >= rhs))
            })
            .count();
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut input = bad_input::BadInput::new(reader);
        let answer = input
            .lines()
            .map(|l| l.split(" ").map(|e| e.parse::<u32>()).collect::<Vec<_>>())
            .map(|v| {
                let mut possibly_safed_outputs = Vec::new();
                for i in 0..v.len() {
                    let mut output = Vec::new();
                    for (j, e) in v.iter().copied().enumerate() {
                        if i == j {
                            continue;
                        }
                        output.push(e);
                    }
                    possibly_safed_outputs.push(output);
                }
                possibly_safed_outputs.push(v);
                possibly_safed_outputs
            })
            .filter(|vs| {
                vs.iter().any(|v| {
                    v.windows(2).all(|w| {
                        let lhs = w[0];
                        let rhs = w[1];
                        lhs.abs_diff(rhs) >= 1 && lhs.abs_diff(rhs) <= 3
                    }) && (v.iter().is_sorted_by(|lhs, rhs| lhs <= rhs)
                        || v.iter().is_sorted_by(|lhs, rhs| lhs >= rhs))
                })
            })
            .count();
        Ok(answer)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
