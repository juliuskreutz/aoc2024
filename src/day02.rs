pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/02.txt")?;

    println!("Day02 Part1: {}", part1(&input)?);
    println!("Day02 Part2: {}", part2(&input)?);

    Ok(())
}

fn check(report: &[i32]) -> bool {
    let inc = report[0] < report[1];

    for levels in report.windows(2) {
        if inc {
            if levels[0] >= levels[1] || levels[1] - levels[0] > 3 {
                return false;
            }
        } else if levels[0] <= levels[1] || levels[0] - levels[1] > 3 {
            return false;
        }
    }

    true
}

pub fn part1(input: &str) -> anyhow::Result<String> {
    let mut reports = Vec::new();

    for line in input.lines() {
        let mut report = Vec::new();

        for s in line.split_whitespace() {
            let level: i32 = s.parse()?;
            report.push(level);
        }

        reports.push(report);
    }

    let mut count = 0;

    for report in reports {
        if check(&report) {
            count += 1;
        }
    }

    Ok(count.to_string())
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let mut reports = Vec::new();

    for line in input.lines() {
        let mut report = Vec::new();

        for s in line.split_whitespace() {
            let level: i32 = s.parse()?;
            report.push(level);
        }

        reports.push(report);
    }

    let mut count = 0;

    for report in reports.into_iter() {
        for n in 0..report.len() {
            let mut report = report.clone();
            report.remove(n);

            if check(&report) {
                count += 1;
                break;
            }
        }
    }

    Ok(count.to_string())
}
