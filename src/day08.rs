use std::collections::{HashMap, HashSet};

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/08.txt")?;

    println!("Day08 Part1: {}", part1(&input)?);
    println!("Day08 Part2: {}", part2(&input)?);

    Ok(())
}

pub fn part1(input: &str) -> anyhow::Result<String> {
    let grid: Vec<Vec<_>> = input.lines().map(|s| s.chars().collect()).collect();

    let mut antennas: HashMap<_, Vec<_>> = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '.' {
                continue;
            }

            antennas.entry(c).or_default().push((x, y));
        }
    }

    let mut antinodes = HashSet::new();

    for locations in antennas.values() {
        for &(x1, y1) in locations {
            for &(x2, y2) in locations {
                if x1 == x2 && y1 == y2 {
                    continue;
                }

                let ox = x1 as i32 - x2 as i32;
                let oy = y1 as i32 - y2 as i32;

                let x = x1 as i32 + ox;
                let y = y1 as i32 + oy;

                if x < 0 || y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 {
                    continue;
                }

                antinodes.insert((x, y));
            }
        }
    }

    Ok(antinodes.len().to_string())
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let grid: Vec<Vec<_>> = input.lines().map(|s| s.chars().collect()).collect();

    let mut antennas: HashMap<_, Vec<_>> = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '.' {
                continue;
            }

            antennas.entry(c).or_default().push((x, y));
        }
    }

    let mut antinodes = HashSet::new();

    for locations in antennas.values() {
        for &(x1, y1) in locations {
            for &(x2, y2) in locations {
                if x1 == x2 && y1 == y2 {
                    continue;
                }

                let ox = x1 as i32 - x2 as i32;
                let oy = y1 as i32 - y2 as i32;

                let mut x = x1 as i32;
                let mut y = y1 as i32;

                loop {
                    antinodes.insert((x, y));

                    x += ox;
                    y += oy;

                    if x < 0 || y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 {
                        break;
                    }
                }
            }
        }
    }

    Ok(antinodes.len().to_string())
}
