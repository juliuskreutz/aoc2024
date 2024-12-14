use std::collections::{HashMap, HashSet};

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/12.txt")?;

    println!("Day12 Part1: {}", part1(&input)?);
    println!("Day12 Part2: {}", part2(&input)?);

    Ok(())
}

#[tracing::instrument(skip_all)]
pub fn part1(input: &str) -> anyhow::Result<String> {
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut visited = HashSet::new();

    let mut sum = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if visited.contains(&(x, y)) {
                continue;
            }

            let mut area = HashSet::new();
            let mut fence = 0;

            let mut stack = vec![(x, y)];

            while let Some((x, y)) = stack.pop() {
                if area.contains(&(x, y)) || x >= grid[0].len() || y >= grid.len() {
                    continue;
                }

                if grid[y][x] == c {
                    area.insert((x, y));
                    visited.insert((x, y));

                    fence += 4;

                    if area.contains(&(x + 1, y)) {
                        fence -= 2;
                    }

                    if area.contains(&(x, y + 1)) {
                        fence -= 2;
                    }

                    if x >= 1 && area.contains(&(x - 1, y)) {
                        fence -= 2;
                    }

                    if y >= 1 && area.contains(&(x, y - 1)) {
                        fence -= 2;
                    }

                    stack.push((x + 1, y));
                    stack.push((x, y + 1));
                    if x >= 1 {
                        stack.push((x - 1, y));
                    }
                    if y >= 1 {
                        stack.push((x, y - 1));
                    }
                }
            }

            sum += area.len() * fence;
        }
    }

    Ok(sum.to_string())
}

#[tracing::instrument(skip_all)]
pub fn part2(input: &str) -> anyhow::Result<String> {
    let grid: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut visited = HashSet::new();

    let mut sum = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            let x = x as i32;
            let y = y as i32;

            if visited.contains(&(x, y)) {
                continue;
            }

            let mut area = 0;
            let mut direction_sides: HashMap<_, Vec<_>> = HashMap::new();

            let mut stack = vec![(x, y)];

            while let Some((x, y)) = stack.pop() {
                if visited.contains(&(x, y)) {
                    continue;
                }
                visited.insert((x, y));

                area += 1;

                for (ox, oy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let x2 = x + ox;
                    let y2 = y + oy;

                    if (0..grid[0].len() as i32).contains(&x2)
                        && (0..grid.len() as i32).contains(&y2)
                        && grid[y2 as usize][x2 as usize] == c
                    {
                        stack.push((x2, y2));
                    } else {
                        direction_sides.entry((ox, oy)).or_default().push((x, y));
                    }
                }
            }

            let mut sides = 0;

            for s in direction_sides.values() {
                let mut visited = HashSet::new();

                for &(x, y) in s {
                    if visited.contains(&(x, y)) {
                        continue;
                    }

                    sides += 1;

                    let mut stack = vec![(x, y)];

                    while let Some((x1, y1)) = stack.pop() {
                        if visited.contains(&(x1, y1)) {
                            continue;
                        }
                        visited.insert((x1, y1));

                        for (ox, oy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                            let x2 = x1 + ox;
                            let y2 = y1 + oy;

                            if s.contains(&(x2, y2)) {
                                stack.push((x2, y2));
                            }
                        }
                    }
                }
            }

            sum += area * sides;
        }
    }

    Ok(sum.to_string())
}
