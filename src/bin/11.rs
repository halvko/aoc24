use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input).unwrap();

        let mut input = input
            .trim()
            .split(' ')
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        for _ in 0..25 {
            let mut next_input = Vec::new();
            for e in input {
                let digits_reversed = {
                    let mut res = Vec::new();
                    let mut e = e;
                    while e > 0 {
                        res.push((e % 10) as u8);
                        e /= 10
                    }
                    res
                };
                if e == 0 {
                    next_input.push(1);
                } else if (digits_reversed.len() & 1) == 0 {
                    // These are flipped because the digits are reversed
                    let (rhs, lhs) = digits_reversed.split_at(digits_reversed.len() / 2);
                    let [lhs, rhs] = [lhs, rhs].map(|ds| {
                        // Normally we should reverse ds to put the least power on the lowest order
                        // bit, but since it is already reversed, we don't need to;
                        ds.iter()
                            .enumerate()
                            .map(|(i, d)| (*d as usize) * 10usize.pow(u32::try_from(i).unwrap()))
                            .sum::<usize>()
                    });
                    next_input.push(lhs);
                    next_input.push(rhs);
                } else {
                    next_input.push(e * 2024)
                }
            }
            input = next_input;
        }
        Ok(input.len())
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input).unwrap();

        let mut stones = HashMap::<usize, usize>::new();

        for e in input.trim().split(' ').map(|s| s.parse::<usize>().unwrap()) {
            *stones.entry(e).or_default() += 1;
        }

        for _ in 0..75 {
            let mut next_stones = HashMap::new();

            for (kind, count) in stones {
                let digits_reversed = {
                    let mut res = Vec::new();
                    let mut e = kind;
                    while e > 0 {
                        res.push((e % 10) as u8);
                        e /= 10
                    }
                    res
                };
                if kind == 0 {
                    *next_stones.entry(1).or_default() += count;
                } else if (digits_reversed.len() & 1) == 0 {
                    // These are flipped because the digits are reversed
                    let (rhs, lhs) = digits_reversed.split_at(digits_reversed.len() / 2);
                    let [lhs, rhs] = [lhs, rhs].map(|ds| {
                        // Normally we should reverse ds to put the least power on the lowest order
                        // bit, but since it is already reversed, we don't need to;
                        ds.iter()
                            .enumerate()
                            .map(|(i, d)| (*d as usize) * 10usize.pow(u32::try_from(i).unwrap()))
                            .sum::<usize>()
                    });
                    *next_stones.entry(lhs).or_default() += count;
                    *next_stones.entry(rhs).or_default() += count;
                } else {
                    *next_stones.entry(kind * 2024).or_default() += count;
                }
            }
            stones = next_stones;
        }
        Ok(stones.values().sum())
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
