use adv_code_2024::*;
use anyhow::*;
use bad_input::BadInput;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut input = BadInput::new(reader);
        let mut lines = input.lines();
        let mut map = HashMap::<u32, HashSet<u32>>::new();
        for l in lines.by_ref() {
            if l.is_empty() {
                break;
            }
            let [lhs, rhs] = l.split_n("|").map(|e| e.parse::<u32>());
            map.entry(rhs).or_default().insert(lhs);
        }

        let answer = lines
            .map(|l| l.split(",").map(|s| s.parse::<u32>()).collect::<Vec<_>>())
            .filter(|pages| {
                let mut must_not_see = HashSet::new();
                for page in pages {
                    if must_not_see.contains(page) {
                        return false;
                    }
                    if let Some(s) = map.get(page) {
                        for e in s.iter() {
                            must_not_see.insert(e);
                        }
                    }
                }
                true
            })
            .map(|l| l[l.len() / 2])
            .sum::<u32>();
        Ok(answer.try_into().unwrap())
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);

    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut input = BadInput::new(reader);
        let mut lines = input.lines();
        let mut map = HashMap::<u32, HashSet<u32>>::new();
        for l in lines.by_ref() {
            if l.is_empty() {
                break;
            }
            let [lhs, rhs] = l.split_n("|").map(|e| e.parse::<u32>());
            map.entry(rhs).or_default().insert(lhs);
        }

        let answer = lines
            .map(|l| l.split(",").map(|s| s.parse::<u32>()).collect::<Vec<_>>())
            .filter_map(|mut pages| {
                let mut has_been_reordered = false;
                let mut reordering = true;
                'outer: while reordering {
                    let mut must_not_see = HashSet::new();
                    for page in 0..(pages.len()) {
                        if must_not_see.contains(&pages[page]) {
                            has_been_reordered = true;
                            pages.swap(page, page - 1);
                            continue 'outer;
                        }
                        if let Some(s) = map.get(&pages[page]) {
                            for e in s.iter() {
                                must_not_see.insert(e);
                            }
                        }
                    }
                    reordering = false;
                }
                if has_been_reordered {
                    Some(pages)
                } else {
                    None
                }
            })
            .map(|l| l[l.len() / 2])
            .sum::<u32>();
        Ok(answer.try_into().unwrap())
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);

    Ok(())
}
