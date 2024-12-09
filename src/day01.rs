use std::collections::HashMap;

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/01.txt")?;

    println!("Day01 Part1: {}", part1(&input)?);
    println!("Day01 Part2: {}", part2(&input)?);

    Ok(())
}

struct Parsed {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn parse(input: &str) -> anyhow::Result<Parsed> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut s = line.split_whitespace();

        let l: i32 = s.next().unwrap().parse()?;
        let r: i32 = s.next().unwrap().parse()?;

        left.push(l);
        right.push(r);
    }

    Ok(Parsed { left, right })
}

#[tracing::instrument(skip_all)]
pub fn part1(input: &str) -> anyhow::Result<String> {
    let Parsed {
        mut left,
        mut right,
    } = parse(input)?;

    left.sort();
    right.sort();

    let mut sum = 0;

    for (l, r) in left.into_iter().zip(right) {
        sum += (l - r).abs();
    }

    Ok(sum.to_string())
}

#[tracing::instrument(skip_all)]
pub fn part2(input: &str) -> anyhow::Result<String> {
    let Parsed { left, right } = parse(input)?;

    let mut counts: HashMap<_, i32> = HashMap::new();

    for r in right {
        *counts.entry(r).or_default() += 1;
    }

    let mut score = 0;

    for i in left {
        score += i * counts.get(&i).copied().unwrap_or_default();
    }

    Ok(score.to_string())
}
