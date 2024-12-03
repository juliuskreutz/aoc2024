use std::collections::HashMap;

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/01.txt")?;

    println!("Day01 Part1: {}", part1(&input)?);
    println!("Day01 Part2: {}", part2(&input)?);

    Ok(())
}

pub fn part1(input: &str) -> anyhow::Result<String> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in input.lines() {
        let mut s = line.split_whitespace();

        let l: i32 = s.next().unwrap().parse()?;
        let r: i32 = s.next().unwrap().parse()?;

        left.push(l);
        right.push(r);
    }

    left.sort();
    right.sort();

    let mut sum = 0;

    for (l, r) in left.into_iter().zip(right) {
        sum += (l - r).abs();
    }

    Ok(sum.to_string())
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let mut list = Vec::new();
    let mut counts: HashMap<usize, usize> = HashMap::new();

    for line in input.lines() {
        let mut s = line.split_whitespace();

        let l: usize = s.next().unwrap().parse()?;
        let r: usize = s.next().unwrap().parse()?;

        list.push(l);

        *counts.entry(r).or_default() += 1;
    }

    let mut score = 0;

    for i in list {
        score += i * counts.get(&i).copied().unwrap_or_default();
    }

    Ok(score.to_string())
}
