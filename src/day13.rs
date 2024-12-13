pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/13.txt")?;

    println!("Day13 Part1: {}", part1(&input)?);
    println!("Day13 Part2: {}", part2(&input)?);

    Ok(())
}

struct Parsed {
    machines: Vec<Machine>,
}

#[derive(Debug)]
struct Machine {
    a: (i64, i64),
    b: (i64, i64),
    n: (i64, i64),
}

fn parse(input: &str) -> anyhow::Result<Parsed> {
    let mut machines = Vec::new();

    for machine_s in input.split("\n\n") {
        let mut split = machine_s.split('\n');

        let a_s = split.next().unwrap();
        let b_s = split.next().unwrap();
        let n_s = split.next().unwrap();

        let mut split = a_s.strip_prefix("Button A: ").unwrap().split(", ");
        let a_x = split.next().unwrap().strip_prefix("X+").unwrap().parse()?;
        let a_y = split.next().unwrap().strip_prefix("Y+").unwrap().parse()?;
        let a = (a_x, a_y);

        let mut split = b_s.strip_prefix("Button B: ").unwrap().split(", ");
        let b_x = split.next().unwrap().strip_prefix("X+").unwrap().parse()?;
        let b_y = split.next().unwrap().strip_prefix("Y+").unwrap().parse()?;
        let b = (b_x, b_y);

        let mut split = n_s.strip_prefix("Prize: ").unwrap().split(", ");
        let n_x = split.next().unwrap().strip_prefix("X=").unwrap().parse()?;
        let n_y = split.next().unwrap().strip_prefix("Y=").unwrap().parse()?;
        let n = (n_x, n_y);

        machines.push(Machine { a, b, n });
    }

    Ok(Parsed { machines })
}

#[tracing::instrument(skip_all)]
pub fn part1(input: &str) -> anyhow::Result<String> {
    let Parsed { machines } = parse(input)?;

    let mut sum = 0;

    for Machine {
        a: (a_x, a_y),
        b: (b_x, b_y),
        n: (n_x, n_y),
    } in machines
    {
        if let Some(min) = calculate_minimum(a_x, a_y, b_x, b_y, n_x, n_y) {
            sum += min;
        }
    }

    Ok(sum.to_string())
}

fn calculate_minimum(a_x: i64, a_y: i64, b_x: i64, b_y: i64, n_x: i64, n_y: i64) -> Option<i64> {
    // cramer
    let det = a_x * b_y - a_y * b_x;
    if det == 0 {
        return None;
    }

    let a = (n_x * b_y - n_y * b_x) / det;
    let b = (a_x * n_y - a_y * n_x) / det;

    if a_x * a + b_x * b == n_x && a_y * a + b_y * b == n_y {
        Some(3 * a + b)
    } else {
        None
    }
}

#[tracing::instrument(skip_all)]
pub fn part2(input: &str) -> anyhow::Result<String> {
    let Parsed { machines } = parse(input)?;

    let mut sum = 0;

    for Machine {
        a: (a_x, a_y),
        b: (b_x, b_y),
        n: (n_x, n_y),
    } in machines
    {
        if let Some(min) = calculate_minimum(
            a_x,
            a_y,
            b_x,
            b_y,
            n_x + 10000000000000,
            n_y + 10000000000000,
        ) {
            sum += min;
        }
    }

    Ok(sum.to_string())
}
