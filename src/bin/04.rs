use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::once;

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"; // TODO: Add the test input

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        fn is_xmas(mut i: impl Iterator<Item = u8>) -> bool {
            let Some(b'X') = i.next() else { return false };
            let Some(b'M') = i.next() else { return false };
            let Some(b'A') = i.next() else { return false };
            let Some(b'S') = i.next() else { return false };
            true
        }

        let lines = reader
            .lines()
            .map(|l| l.unwrap())
            .map(|s| s.into_bytes())
            .collect::<Vec<_>>();

        let mut count = 0;
        for (y, l) in lines.iter().enumerate() {
            for (x, _) in l.iter().enumerate() {
                if lines.len() > x + 3 {
                    if is_xmas(l[x..(x + 4)].iter().copied()) {
                        count += 1
                    }
                    if is_xmas(l[x..(x + 4)].iter().rev().copied()) {
                        count += 1
                    }
                }
                if lines.len() > y + 3 {
                    if is_xmas(
                        once(lines[y][x])
                            .chain(once(lines[y + 1][x]))
                            .chain(once(lines[y + 2][x]))
                            .chain(once(lines[y + 3][x])),
                    ) {
                        count += 1
                    }
                    if l.len() > x + 3
                        && is_xmas(
                            once(lines[y][x])
                                .chain(once(lines[y + 1][x + 1]))
                                .chain(once(lines[y + 2][x + 2]))
                                .chain(once(lines[y + 3][x + 3])),
                        )
                    {
                        count += 1
                    }
                    if x >= 3
                        && is_xmas(
                            once(lines[y][x])
                                .chain(once(lines[y + 1][x - 1]))
                                .chain(once(lines[y + 2][x - 2]))
                                .chain(once(lines[y + 3][x - 3])),
                        )
                    {
                        count += 1
                    }
                }
                if y >= 3 {
                    if is_xmas(
                        once(lines[y][x])
                            .chain(once(lines[y - 1][x]))
                            .chain(once(lines[y - 2][x]))
                            .chain(once(lines[y - 3][x])),
                    ) {
                        count += 1
                    }
                    if l.len() > x + 3
                        && is_xmas(
                            once(lines[y][x])
                                .chain(once(lines[y - 1][x + 1]))
                                .chain(once(lines[y - 2][x + 2]))
                                .chain(once(lines[y - 3][x + 3])),
                        )
                    {
                        count += 1
                    }
                    if x >= 3
                        && is_xmas(
                            once(lines[y][x])
                                .chain(once(lines[y - 1][x - 1]))
                                .chain(once(lines[y - 2][x - 2]))
                                .chain(once(lines[y - 3][x - 3])),
                        )
                    {
                        count += 1
                    }
                }
            }
        }
        // TODO: Solve Part 1 of the puzzle
        let answer = count;
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = reader
            .lines()
            .map(|l| l.unwrap())
            .map(|s| s.into_bytes())
            .collect::<Vec<_>>();

        let mut count = 0;
        for ls in lines.windows(3) {
            for ws in ls[0]
                .windows(3)
                .zip(ls[1].windows(3))
                .zip(ls[2].windows(3))
                .map(|((a, b), c)| [a, b, c])
            {
                if ws[1][1] != b'A' {
                    continue;
                }
                if ws[0][0] == b'M'
                    && ws[2][2] == b'S'
                    && ((ws[0][2] == b'M' && ws[2][0] == b'S')
                        || (ws[2][0] == b'M' && ws[0][2] == b'S'))
                {
                    count += 1;
                    continue;
                }
                if ws[2][2] == b'M'
                    && ws[0][0] == b'S'
                    && ((ws[0][2] == b'M' && ws[2][0] == b'S')
                        || (ws[2][0] == b'M' && ws[0][2] == b'S'))
                {
                    count += 1;
                    continue;
                }
            }
        }
        Ok(count)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
