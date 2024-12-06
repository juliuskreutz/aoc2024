use std::collections::HashSet;

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/06.txt")?;

    println!("Day06 Part1: {}", part1(&input)?);
    println!("Day06 Part2: {}", part2(&input)?);

    Ok(())
}

pub fn part1(input: &str) -> anyhow::Result<String> {
    let grid: Vec<Vec<_>> = input.lines().map(|s| s.chars().collect()).collect();

    let mut guard = (0, 0);

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '^' {
                guard = (x as i32, y as i32);
            }
        }
    }

    let mut visited = HashSet::new();
    visited.insert((guard.0, guard.1));

    let mut direction = (0, -1);

    loop {
        let next = ((guard.0 + direction.0), (guard.1 + direction.1));

        if next.0 < 0
            || next.1 < 0
            || next.0 as usize >= grid[0].len()
            || next.1 as usize >= grid.len()
        {
            break;
        }

        if grid[next.1 as usize][next.0 as usize] == '#' {
            direction = match direction {
                (0, -1) => (1, 0),
                (1, 0) => (0, 1),
                (0, 1) => (-1, 0),
                (-1, 0) => (0, -1),
                _ => unreachable!(),
            };
        } else {
            guard = next;
            visited.insert(guard);
        }
    }

    Ok(visited.len().to_string())
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let grid: Vec<Vec<_>> = input.lines().map(|s| s.chars().collect()).collect();

    let mut guard = (0, 0);

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '^' {
                guard = (x as i32, y as i32);
            }
        }
    }

    let mut count = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '.' {
                let mut grid = grid.clone();
                grid[y][x] = '#';

                let mut guard = guard;
                let mut direction = (0, -1);

                let mut visited = HashSet::new();
                visited.insert((guard, direction));

                loop {
                    let next = ((guard.0 + direction.0), (guard.1 + direction.1));

                    if next.0 < 0
                        || next.1 < 0
                        || next.0 as usize >= grid[0].len()
                        || next.1 as usize >= grid.len()
                    {
                        break;
                    }

                    if grid[next.1 as usize][next.0 as usize] == '#' {
                        direction = match direction {
                            (0, -1) => (1, 0),
                            (1, 0) => (0, 1),
                            (0, 1) => (-1, 0),
                            (-1, 0) => (0, -1),
                            _ => unreachable!(),
                        };
                    } else {
                        guard = next;
                    }

                    if visited.contains(&(guard, direction)) {
                        count += 1;
                        break;
                    }

                    visited.insert((guard, direction));
                }
            }
        }
    }

    Ok(count.to_string())
}
