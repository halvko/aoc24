use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mul_regex = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();

        let mut input = String::new();
        reader.read_to_string(&mut input).unwrap();
        let answer = mul_regex
            .find_iter(&input)
            .map(|m| {
                let captures = mul_regex.captures(m.as_str()).unwrap();
                let lhs = captures[1].parse::<usize>().unwrap();
                let rhs = captures[2].parse::<usize>().unwrap();
                lhs * rhs
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mul_regex =
            Regex::new(r"(?:(mul)\(([0-9]+),([0-9]+)\))|(?:(do)\(\))|(?:(don\'t)\(\))").unwrap();

        let mut input = String::new();
        reader.read_to_string(&mut input).unwrap();
        let mut enabled = true;
        let answer = mul_regex
            .captures_iter(&input)
            .filter_map(|captures| {
                match captures
                    .get(1)
                    .or(captures.get(4))
                    .or(captures.get(5))
                    .unwrap()
                    .as_str()
                {
                    "do" => enabled = true,
                    "don't" => enabled = false,
                    "mul" => {
                        if !enabled {
                            return None;
                        }
                        let lhs = captures[2].parse::<usize>().unwrap();
                        let rhs = captures[3].parse::<usize>().unwrap();
                        return Some(lhs * rhs);
                    }
                    s => panic!("{s}"),
                }
                None
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
