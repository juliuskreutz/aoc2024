use std::collections::VecDeque;

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/15.txt")?;

    println!("Day15 Part1: {}", part1(&input)?);
    println!("Day15 Part2: {}", part2(&input)?);

    Ok(())
}

struct Parsed {
    grid: Vec<Vec<char>>,
    moves: Vec<char>,
}

fn parse(input: &str) -> anyhow::Result<Parsed> {
    let mut split = input.split("\n\n");

    let grid = split
        .next()
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let moves = split
        .next()
        .unwrap()
        .lines()
        .flat_map(|l| l.chars())
        .collect();

    Ok(Parsed { grid, moves })
}

#[tracing::instrument(skip_all)]
pub fn part1(input: &str) -> anyhow::Result<String> {
    let Parsed { mut grid, moves } = parse(input)?;

    let mut robot = (0, 0);

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '@' {
                robot = (x as i32, y as i32);
            }
        }
    }

    'outer: for m in moves {
        let (x, y) = robot;

        let (ox, oy) = match m {
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            '^' => (0, -1),
            _ => unreachable!(),
        };

        let x1 = x + ox;
        let y1 = y + oy;

        if grid[y1 as usize][x1 as usize] == '#' {
            continue;
        } else if grid[y1 as usize][x1 as usize] == '.' {
            grid[y1 as usize][x1 as usize] = '@';
            grid[y as usize][x as usize] = '.';
            robot = (x1, y1);
            continue 'outer;
        }

        let mut queue = VecDeque::new();
        queue.push_back((x, y));
        let mut push = Vec::new();

        while let Some((x, y)) = queue.pop_front() {
            push.push((x, y));

            let x1 = x + ox;
            let y1 = y + oy;

            match grid[y1 as usize][x1 as usize] {
                '#' => {
                    continue 'outer;
                }
                'O' => {
                    queue.push_back((x1, y1));
                }
                _ => {}
            }
        }

        for &(x1, y1) in push.iter().rev() {
            let x2 = x1 + ox;
            let y2 = y1 + oy;

            grid[y2 as usize][x2 as usize] = grid[y1 as usize][x1 as usize];
            // we can assume, that we'll always be swapping with a .
            grid[y1 as usize][x1 as usize] = '.';
        }

        robot = (x1, y1);
    }

    let mut sum = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'O' {
                sum += x + y * 100;
            }
        }
    }

    Ok(sum.to_string())
}

#[tracing::instrument(skip_all)]
pub fn part2(input: &str) -> anyhow::Result<String> {
    let Parsed { mut grid, moves } = parse(input)?;

    let mut new_grid = Vec::new();

    for row in &grid {
        let mut new_row = Vec::new();

        for &c in row {
            match c {
                '.' => {
                    new_row.push('.');
                    new_row.push('.');
                }
                '#' => {
                    new_row.push('#');
                    new_row.push('#');
                }
                '@' => {
                    new_row.push('@');
                    new_row.push('.');
                }
                'O' => {
                    new_row.push('[');
                    new_row.push(']');
                }
                _ => {}
            }
        }

        new_grid.push(new_row);
    }

    grid = new_grid;

    let mut robot = (0, 0);

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '@' {
                robot = (x as i32, y as i32);
            }
        }
    }

    'outer: for m in moves {
        let (x, y) = robot;

        let (ox, oy) = match m {
            '>' => (1, 0),
            'v' => (0, 1),
            '<' => (-1, 0),
            '^' => (0, -1),
            _ => unreachable!(),
        };

        let x1 = x + ox;
        let y1 = y + oy;

        if grid[y1 as usize][x1 as usize] == '#' {
            continue;
        } else if grid[y1 as usize][x1 as usize] == '.' {
            grid[y1 as usize][x1 as usize] = '@';
            grid[y as usize][x as usize] = '.';
            robot = (x1, y1);
            continue 'outer;
        }

        let mut queue = VecDeque::new();
        queue.push_back((x, y));
        let mut push = Vec::new();

        while let Some((x, y)) = queue.pop_front() {
            // easier to handle []
            if push.contains(&(x, y)) {
                continue;
            }
            push.push((x, y));

            let x1 = x + ox;
            let y1 = y + oy;

            match grid[y1 as usize][x1 as usize] {
                '#' => {
                    continue 'outer;
                }
                '[' => {
                    queue.push_back((x1, y1));
                    queue.push_back((x1 + 1, y1));
                }
                ']' => {
                    queue.push_back((x1, y1));
                    queue.push_back((x1 - 1, y1));
                }
                _ => {}
            }
        }

        for &(x1, y1) in push.iter().rev() {
            let x2 = x1 + ox;
            let y2 = y1 + oy;

            grid[y2 as usize][x2 as usize] = grid[y1 as usize][x1 as usize];
            // we can assume, that we'll always be swapping with a .
            grid[y1 as usize][x1 as usize] = '.';
        }

        robot = (x1, y1);
    }

    let mut sum = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '[' {
                sum += x + y * 100;
            }
        }
    }

    Ok(sum.to_string())
}
