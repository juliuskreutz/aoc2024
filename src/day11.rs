use std::collections::HashMap;

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/11.txt")?;

    println!("Day11 Part1: {}", part1(&input)?);
    println!("Day11 Part2: {}", part2(&input)?);

    Ok(())
}

fn blink(
    stone: usize,
    i: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> anyhow::Result<usize> {
    if let Some(&len) = cache.get(&(stone, i)) {
        return Ok(len);
    }

    if i == 0 {
        return Ok(1);
    }

    let len = match stone {
        0 => blink(1, i - 1, cache)?,
        stone => {
            let stone_s = stone.to_string();

            if stone_s.len() % 2 == 0 {
                let (left_s, right_s) = stone_s.split_at(stone_s.len() / 2);

                let left = left_s.parse()?;
                let right = right_s.parse()?;

                blink(left, i - 1, cache)? + blink(right, i - 1, cache)?
            } else {
                blink(stone * 2024, i - 1, cache)?
            }
        }
    };

    cache.insert((stone, i), len);

    Ok(len)
}

#[tracing::instrument(skip_all)]
pub fn part1(input: &str) -> anyhow::Result<String> {
    let mut stones: Vec<usize> = Vec::new();

    for line in input.split_whitespace() {
        stones.push(line.parse()?);
    }

    let mut sum = 0;
    let mut cache = HashMap::new();

    for stone in stones {
        sum += blink(stone, 25, &mut cache)?;
    }

    Ok(sum.to_string())
}

#[tracing::instrument(skip_all)]
pub fn part2(input: &str) -> anyhow::Result<String> {
    let mut stones: Vec<usize> = Vec::new();

    for line in input.split_whitespace() {
        stones.push(line.parse()?);
    }

    let mut sum = 0;
    let mut cache = HashMap::new();

    for stone in stones {
        sum += blink(stone, 75, &mut cache)?;
    }

    Ok(sum.to_string())
}
