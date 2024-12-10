use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader
            .lines()
            .map(|l| {
                let mut b = l.unwrap().into_bytes();
                b.iter_mut().for_each(|b| *b -= b'0');
                b
            })
            .collect::<Vec<_>>();

        let mut summits: usize = 0;
        for (coord, _) in input
            .iter()
            .enumerate()
            .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, v)| ([x, y], *v)))
            .filter(|(_, v)| *v == 0)
        {
            let mut seen = HashSet::new();
            let mut work_queue = VecDeque::new();
            work_queue.push_back(coord);
            while let Some([x, y]) = work_queue.pop_front() {
                if !seen.insert([x, y]) {
                    continue;
                }
                let v = input[y][x];
                if v == 9 {
                    summits += 1;
                }
                for offset in [[1, 0], [0, 1], [-1, 0], [0, -1]] {
                    let Some(new_coord) = x
                        .checked_add_signed(offset[0])
                        .and_then(|x| y.checked_add_signed(offset[1]).map(move |y| [x, y]))
                    else {
                        continue;
                    };
                    let Some(r) = input.get(new_coord[1]) else {
                        continue;
                    };
                    let Some(&new_v) = r.get(new_coord[0]) else {
                        continue;
                    };
                    if new_v != v + 1 {
                        continue;
                    }
                    work_queue.push_back(new_coord);
                }
            }
        }

        // TODO: Solve Part 1 of the puzzle
        Ok(summits)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input = reader
            .lines()
            .map(|l| {
                let mut b = l.unwrap().into_bytes();
                b.iter_mut().for_each(|b| *b -= b'0');
                b
            })
            .collect::<Vec<_>>();

        let mut trails: usize = 0;
        for (coord, _) in input
            .iter()
            .enumerate()
            .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, v)| ([x, y], *v)))
            .filter(|(_, v)| *v == 0)
        {
            let mut work_stack = Vec::new();
            work_stack.push(coord);
            while let Some([x, y]) = work_stack.pop() {
                let v = input[y][x];
                if v == 9 {
                    trails += 1;
                }
                for offset in [[1, 0], [0, 1], [-1, 0], [0, -1]] {
                    let Some(new_coord) = x
                        .checked_add_signed(offset[0])
                        .and_then(|x| y.checked_add_signed(offset[1]).map(move |y| [x, y]))
                    else {
                        continue;
                    };
                    let Some(r) = input.get(new_coord[1]) else {
                        continue;
                    };
                    let Some(&new_v) = r.get(new_coord[0]) else {
                        continue;
                    };
                    if new_v != v + 1 {
                        continue;
                    }
                    work_stack.push(new_coord);
                }
            }
        }

        Ok(trails)
    }

    assert_eq!(81, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
