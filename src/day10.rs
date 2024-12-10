use std::collections::HashSet;

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/10.txt")?;

    println!("Day10 Part1: {}", part1(&input)?);
    println!("Day10 Part2: {}", part2(&input)?);

    Ok(())
}

#[tracing::instrument(skip_all)]
pub fn part1(input: &str) -> anyhow::Result<String> {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let mut trail_heads = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, &i) in row.iter().enumerate() {
            if i == 0 {
                trail_heads.push((x, y));
            }
        }
    }

    let mut sum = 0;

    for trail_head in trail_heads {
        let mut visited = HashSet::new();

        let mut stack = Vec::new();
        stack.push(trail_head);

        while let Some((x, y)) = stack.pop() {
            if visited.contains(&(x, y)) {
                continue;
            }
            visited.insert((x, y));

            let v = grid[y][x];

            if v == 9 {
                sum += 1;
                continue;
            }

            if y >= 1 && grid[y - 1][x] - v == 1 {
                stack.push((x, y - 1));
            }

            if y < grid.len() - 1 && grid[y + 1][x] - v == 1 {
                stack.push((x, y + 1));
            }

            if x >= 1 && grid[y][x - 1] - v == 1 {
                stack.push((x - 1, y));
            }

            if x < grid[0].len() - 1 && grid[y][x + 1] - v == 1 {
                stack.push((x + 1, y));
            }
        }
    }

    Ok(sum.to_string())
}

#[tracing::instrument(skip_all)]
pub fn part2(input: &str) -> anyhow::Result<String> {
    let grid: Vec<Vec<_>> = input
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();

    let mut trail_heads = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, &i) in row.iter().enumerate() {
            if i == 0 {
                trail_heads.push((x, y));
            }
        }
    }

    let mut sum = 0;

    for trail_head in trail_heads {
        let mut stack = Vec::new();
        stack.push(trail_head);

        while let Some((x, y)) = stack.pop() {
            let v = grid[y][x];

            if v == 9 {
                sum += 1;
                continue;
            }

            if y >= 1 && grid[y - 1][x] - v == 1 {
                stack.push((x, y - 1));
            }

            if y < grid.len() - 1 && grid[y + 1][x] - v == 1 {
                stack.push((x, y + 1));
            }

            if x >= 1 && grid[y][x - 1] - v == 1 {
                stack.push((x - 1, y));
            }

            if x < grid[0].len() - 1 && grid[y][x + 1] - v == 1 {
                stack.push((x + 1, y));
            }
        }
    }

    Ok(sum.to_string())
}
