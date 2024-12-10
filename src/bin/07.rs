use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn main() -> Result<()> {
    start_day(DAY);

    fn add_one<const N: u8>(v: &mut [u8]) -> bool {
        for val in v.iter_mut().rev() {
            if *val < N {
                *val += 1;
                return true;
            }
            *val = 0;
        }
        false
    }

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                use std::iter;
                let (total, values) = l.split_once(": ").unwrap();
                let mut parsed = iter::once(total)
                    .chain(values.split(' '))
                    .map(|v| v.parse::<usize>().unwrap());
                let total = parsed.next().unwrap();
                let values = parsed.collect::<Vec<_>>();
                (total, values)
            })
            .filter(|(total, values)| {
                let mut operations = vec![0; values.len() - 1];
                while {
                    // BE WARE: DO WHILE
                    let mut values_iter = values.iter();
                    let first = values_iter.next().unwrap();
                    if values_iter
                        .zip(operations.iter())
                        .fold(*first, |acc, (&v, &o)| match o {
                            0 => acc + v,
                            1 => acc * v,
                            _ => unreachable!(),
                        })
                        == *total
                    {
                        return true;
                    }
                    add_one::<1>(&mut operations)
                } {}
                false
            })
            .map(|(t, _)| t)
            .sum();
        Ok(answer)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let answer = reader
            .lines()
            .map(|l| l.unwrap())
            .map(|l| {
                use std::iter;
                let (total, values) = l.split_once(": ").unwrap();
                let mut parsed = iter::once(total)
                    .chain(values.split(' '))
                    .map(|v| v.parse::<usize>().unwrap());
                let total = parsed.next().unwrap();
                let values = parsed.collect::<Vec<_>>();
                (total, values)
            })
            .filter(|(total, values)| {
                let mut operations = vec![0; values.len() - 1];
                while {
                    // BE WARE: DO WHILE
                    let mut values_iter = values.iter();
                    let first = values_iter.next().unwrap();
                    if values_iter
                        .zip(operations.iter())
                        .fold(*first, |acc, (&v, &o)| match o {
                            0 => acc + v,
                            1 => acc * v,
                            2 => {
                                let mut digits = 1;
                                let mut value = v;
                                while {
                                    value /= 10;
                                    value > 0
                                } {
                                    digits += 1;
                                }
                                let mut acc = acc;
                                acc *= 10usize.pow(digits);
                                acc + v
                            }
                            _ => unreachable!(),
                        })
                        == *total
                    {
                        return true;
                    }
                    add_one::<2>(&mut operations)
                } {}
                false
            })
            .map(|(t, _)| t)
            .sum();
        Ok(answer)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
