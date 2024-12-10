use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<u32> {
        let (mut left, mut right) = reader
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                let split = l.split_once(|c: char| c.is_whitespace()).unwrap();
                (
                    split.0.trim().parse::<u32>().unwrap(),
                    split.1.trim().parse::<u32>().unwrap(),
                )
            })
            .collect::<(Vec<_>, Vec<_>)>();
        left.sort_unstable();
        right.sort_unstable();

        let answer = left
            .iter()
            .zip(right.iter())
            .map(|(l, r)| l.abs_diff(*r))
            .sum();
        Ok(answer)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<u32> {
        let (left, right) = reader
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                let split = l.split_once(|c: char| c.is_whitespace()).unwrap();
                (
                    split.0.trim().parse::<u32>().unwrap(),
                    split.1.trim().parse::<u32>().unwrap(),
                )
            })
            .fold(
                (Vec::new(), HashMap::<u32, u32>::new()),
                |(mut v, mut map), (left, right)| {
                    v.push(left);
                    *map.entry(right).or_default() += 1;
                    (v, map)
                },
            );

        let result = left
            .iter()
            .map(|n| right.get(n).copied().unwrap_or_default() * n)
            .sum();
        Ok(result)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    Ok(())
}
