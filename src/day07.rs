pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/07.txt")?;

    println!("Day07 Part1: {}", part1(&input)?);
    println!("Day07 Part2: {}", part2(&input)?);

    Ok(())
}

#[tracing::instrument(skip_all)]
pub fn part1(input: &str) -> anyhow::Result<String> {
    let mut sum = 0;

    for line in input.lines() {
        let mut split = line.split(':');

        let n: usize = split.next().unwrap().parse()?;

        let mut values: Vec<usize> = Vec::new();

        for s in split.next().unwrap().split_whitespace() {
            values.push(s.parse()?);
        }

        let mut results = vec![values[0]];

        for &v in values.iter().skip(1) {
            let old_results = results.clone();
            results.clear();

            for result in old_results {
                let add = v + result;
                let mul = v * result;

                if add <= n {
                    results.push(add);
                }

                if mul <= n {
                    results.push(mul);
                }
            }
        }

        if results.contains(&n) {
            sum += n;
        }
    }

    Ok(sum.to_string())
}

#[tracing::instrument(skip_all)]
pub fn part2(input: &str) -> anyhow::Result<String> {
    let mut sum = 0;

    for line in input.lines() {
        let mut split = line.split(':');

        let n: usize = split.next().unwrap().parse()?;

        let mut values: Vec<usize> = Vec::new();

        for s in split.next().unwrap().split_whitespace() {
            values.push(s.parse()?);
        }

        let mut results = vec![values[0]];

        for &v in values.iter().skip(1) {
            let rs = results.clone();
            results.clear();

            for result in rs {
                let add = v + result;
                let mul = v * result;
                let combined = format!("{result}{v}").parse()?;

                if add <= n {
                    results.push(add);
                }

                if mul <= n {
                    results.push(mul);
                }

                if combined <= n {
                    results.push(combined);
                }
            }
        }

        if results.contains(&n) {
            sum += n;
        }
    }

    Ok(sum.to_string())
}
