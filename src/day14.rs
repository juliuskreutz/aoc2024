use std::collections::HashSet;

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/14.txt")?;

    println!("Day14 Part1: {}", part1(&input)?);
    println!("Day14 Part2: {}", part2(&input)?);

    Ok(())
}

struct Parsed {
    robots: Vec<Robot>,
}

#[derive(Debug)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

fn parse(input: &str) -> anyhow::Result<Parsed> {
    let mut robots = Vec::new();

    for line in input.lines() {
        let mut split = line.split_whitespace();

        let p_s = split.next().unwrap().strip_prefix("p=").unwrap();
        let mut p_split = p_s.split(',');
        let p_x = p_split.next().unwrap().parse()?;
        let p_y = p_split.next().unwrap().parse()?;
        let p = (p_x, p_y);

        let v_s = split.next().unwrap().strip_prefix("v=").unwrap();
        let mut v_split = v_s.split(',');
        let v_x = v_split.next().unwrap().parse()?;
        let v_y = v_split.next().unwrap().parse()?;
        let v = (v_x, v_y);

        let robot = Robot { p, v };

        robots.push(robot);
    }

    Ok(Parsed { robots })
}

#[tracing::instrument(skip_all)]
pub fn part1(input: &str) -> anyhow::Result<String> {
    let Parsed { mut robots } = parse(input)?;

    let width = 101;
    let height = 103;

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.p.0 = (robot.p.0 + robot.v.0 + width) % width;
            robot.p.1 = (robot.p.1 + robot.v.1 + height) % height;
        }
    }

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;

    for robot in robots {
        let x = robot.p.0;
        let y = robot.p.1;

        let mid_x = width / 2;
        let mid_y = height / 2;

        if x == mid_x || y == mid_y {
            continue;
        }

        match (robot.p.0 > mid_x, robot.p.1 > mid_y) {
            (false, true) => q1 += 1,
            (true, true) => q2 += 1,
            (true, false) => q3 += 1,
            (false, false) => q4 += 1,
        }
    }

    let mul = q1 * q2 * q3 * q4;

    Ok(mul.to_string())
}

#[tracing::instrument(skip_all)]
pub fn part2(input: &str) -> anyhow::Result<String> {
    let Parsed { mut robots } = parse(input)?;

    let width = 101;
    let height = 103;

    let mut i = 0;

    loop {
        i += 1;

        for robot in robots.iter_mut() {
            robot.p.0 = (robot.p.0 + robot.v.0 + width) % width;
            robot.p.1 = (robot.p.1 + robot.v.1 + height) % height;
        }

        let positions: HashSet<_> = robots.iter().map(|r| r.p).collect();
        let mut visited = HashSet::new();

        let mut components = 0;

        for &(x, y) in &positions {
            if visited.contains(&(x, y)) {
                continue;
            }

            components += 1;

            let mut stack = vec![(x, y)];

            while let Some((x, y)) = stack.pop() {
                if visited.contains(&(x, y)) {
                    continue;
                }
                visited.insert((x, y));

                for (ox, oy) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
                    let x = x + ox;
                    let y = y + oy;

                    if positions.contains(&(x, y)) {
                        stack.push((x, y));
                    }
                }
            }
        }

        if components <= 150 {
            //for y in 0..height {
            //    for x in 0..width {
            //        if positions.contains(&(x, y)) {
            //            print!("#");
            //        } else {
            //            print!(".");
            //        }
            //    }
            //
            //    println!();
            //}

            break;
        }
    }

    Ok(i.to_string())
}
