use regex::Regex;

pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/03.txt")?;

    println!("Day03 Part1: {}", part1(&input)?);
    println!("Day03 Part2: {}", part2(&input)?);

    Ok(())
}

#[tracing::instrument(skip_all)]
pub fn part1(input: &str) -> anyhow::Result<String> {
    let mut sum = 0;

    let re: Regex = r"mul\((\d+),(\d+)\)".parse()?;
    for capture in re.captures_iter(input) {
        let x: i32 = capture.get(1).unwrap().as_str().parse()?;
        let y: i32 = capture.get(2).unwrap().as_str().parse()?;

        sum += x * y;
    }

    Ok(sum.to_string())
}

#[tracing::instrument(skip_all)]
pub fn part2(input: &str) -> anyhow::Result<String> {
    let mut sum = 0;

    let mut mul = true;

    let re: Regex = r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)".parse()?;
    for capture in re.captures_iter(input) {
        match capture.get(0).unwrap().as_str() {
            "do()" => {
                mul = true;
                continue;
            }
            "don't()" => {
                mul = false;
                continue;
            }
            _ => {}
        }

        if mul {
            let x: i32 = capture.get(1).unwrap().as_str().parse()?;
            let y: i32 = capture.get(2).unwrap().as_str().parse()?;

            sum += x * y;
        }
    }

    Ok(sum.to_string())
}
