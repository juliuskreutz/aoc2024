use std::collections::HashSet;

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/06.txt")?;

    println!("Day06 Part1: {}", part1(&input)?);
    println!("Day06 Part2: {}", part2(&input)?);

    Ok(())
}

#[tracing::instrument(skip_all)]
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

#[tracing::instrument(skip_all)]
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

    let mut direction = (0, -1);

    let mut visited = HashSet::new();
    visited.insert((guard.0, guard.1));

    let mut obstacles = HashSet::new();

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

            continue;
        }

        let obstacle = next;

        if grid[obstacle.1 as usize][obstacle.0 as usize] == '.'
            && !visited.contains(&obstacle)
            && !obstacles.contains(&obstacle)
        {
            let mut guard = guard;
            let mut direction = direction;

            let mut visited = HashSet::new();
            visited.insert((guard, direction));

            let b = loop {
                let next = ((guard.0 + direction.0), (guard.1 + direction.1));

                if next.0 < 0
                    || next.1 < 0
                    || next.0 as usize >= grid[0].len()
                    || next.1 as usize >= grid.len()
                {
                    break false;
                }

                if grid[next.1 as usize][next.0 as usize] == '#' || next == obstacle {
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
                    break true;
                }

                visited.insert((guard, direction));
            };

            if b {
                obstacles.insert(obstacle);
            }
        }

        guard = next;
        visited.insert(guard);
    }

    Ok(obstacles.len().to_string())
}
