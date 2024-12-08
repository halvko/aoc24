use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // TODO: Solve Part 1 of the puzzle

        let mut antennas = HashMap::<u8, HashSet<[usize; 2]>>::new();
        let map = reader
            .lines()
            .map(Result::unwrap)
            .map(|s| s.into_bytes())
            .collect::<Vec<_>>();
        let y_len = map.len();
        let x_len = map.get(0).map(|r| r.len()).unwrap_or(0);

        for (coord, &v) in map
            .iter()
            .enumerate()
            .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, v)| ([x, y], v)))
        {
            if v == b'.' {
                continue;
            }
            antennas.entry(v).or_default().insert(coord);
        }

        let mut anti_zones = HashSet::new();

        let coord_within = |coord: [usize; 2]| coord[0] < x_len && coord[1] < y_len;

        for (_, coords) in antennas.iter() {
            for coord_1 in coords {
                for coord_2 in coords.iter().filter(|&c| c != coord_1) {
                    let diff = [
                        coord_1[0].wrapping_sub(coord_2[0]),
                        coord_1[1].wrapping_sub(coord_2[1]),
                    ];
                    let maybe_anti_zone = [
                        coord_1[0].wrapping_add(diff[0]),
                        coord_1[1].wrapping_add(diff[1]),
                    ];
                    if !coords.contains(&maybe_anti_zone) && coord_within(maybe_anti_zone) {
                        anti_zones.insert(maybe_anti_zone);
                    }
                }
            }
        }

        Ok(anti_zones.len())
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut antennas = HashMap::<u8, HashSet<[usize; 2]>>::new();
        let map = reader
            .lines()
            .map(Result::unwrap)
            .map(|s| s.into_bytes())
            .collect::<Vec<_>>();
        let y_len = map.len();
        let x_len = map.get(0).map(|r| r.len()).unwrap_or(0);

        for (coord, &v) in map
            .iter()
            .enumerate()
            .flat_map(|(y, r)| r.iter().enumerate().map(move |(x, v)| ([x, y], v)))
        {
            if v == b'.' {
                continue;
            }
            antennas.entry(v).or_default().insert(coord);
        }

        let mut anti_zones = HashSet::new();

        let coord_within = |coord: [usize; 2]| coord[0] < x_len && coord[1] < y_len;

        for (_, coords) in antennas.iter() {
            for coord_1 in coords {
                for coord_2 in coords.iter().filter(|&c| c != coord_1) {
                    let diff = [
                        coord_1[0].wrapping_sub(coord_2[0]),
                        coord_1[1].wrapping_sub(coord_2[1]),
                    ];

                    let mut anti_zone = [coord_1[0], coord_1[1]];
                    while coord_within(anti_zone) {
                        anti_zones.insert(anti_zone);
                        anti_zone = [
                            anti_zone[0].wrapping_add(diff[0]),
                            anti_zone[1].wrapping_add(diff[1]),
                        ];
                    }
                }
            }
        }

        Ok(anti_zones.len())
    }

    assert_eq!(34, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
