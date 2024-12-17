pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/17.txt")?;

    println!("Day17 Part1: {}", part1(&input)?);
    println!("Day17 Part2: {}", part2(&input)?);

    Ok(())
}

struct Parsed {
    regs: [usize; 3],
    program: Vec<usize>,
}

fn parse(input: &str) -> anyhow::Result<Parsed> {
    let mut lines = input.lines();

    let mut regs = [0; 3];
    regs[0] = lines
        .next()
        .unwrap()
        .strip_prefix("Register A: ")
        .unwrap()
        .parse()?;
    regs[1] = lines
        .next()
        .unwrap()
        .strip_prefix("Register B: ")
        .unwrap()
        .parse()?;
    regs[2] = lines
        .next()
        .unwrap()
        .strip_prefix("Register C: ")
        .unwrap()
        .parse()?;

    lines.next();

    let program = lines
        .next()
        .unwrap()
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    Ok(Parsed { regs, program })
}

fn run(regs: &mut [usize], program: &[usize]) -> Vec<usize> {
    fn op(regs: &[usize], n: usize) -> usize {
        match n {
            0..=3 => n,
            i => regs[i - 4],
        }
    }

    let mut i = 0;
    let mut out = Vec::new();

    loop {
        if i >= program.len() {
            break;
        }

        //println!("{i} {} {} {regs:?}", program[i], program[i + 1]);

        match program[i] {
            0 => {
                let n = regs[0];
                let d = 2usize.pow(op(regs, program[i + 1]) as u32);

                regs[0] = n / d;
            }
            1 => {
                let x = regs[1];
                let y = program[i + 1];

                regs[1] = x ^ y;
            }
            2 => {
                let x = op(regs, program[i + 1]);

                regs[1] = x & 0b111;
            }
            3 => {
                if regs[0] != 0 {
                    i = program[i + 1];
                    continue;
                }
            }
            4 => {
                regs[1] ^= regs[2];
            }
            5 => {
                let x = op(regs, program[i + 1]) & 0b111;

                out.push(x);
            }
            6 => {
                let n = regs[0];
                let d = 2usize.pow(op(regs, program[i + 1]) as u32);

                regs[1] = n / d;
            }
            7 => {
                let n = regs[0];
                let d = 2usize.pow(op(regs, program[i + 1]) as u32);

                regs[2] = n / d;
            }
            _ => {}
        }

        i += 2;
    }

    out
}

#[tracing::instrument(skip_all)]
pub fn part1(input: &str) -> anyhow::Result<String> {
    let Parsed { mut regs, program } = parse(input)?;

    let out: Vec<_> = run(&mut regs, &program)
        .into_iter()
        .map(|i| i.to_string())
        .collect();

    Ok(out.join(","))
}

#[tracing::instrument(skip_all)]
pub fn part2(input: &str) -> anyhow::Result<String> {
    // 2,4  B = A & 7
    // 1,2  B ^= 2
    // 7,5  C = A >> B
    // 0,3  A = A >> 3
    // 1,7  B ^= 7
    // 4,1  B ^= C
    // 5,5  out(B & 7)
    // 3,0  jnz 0

    let Parsed { regs: _, program } = parse(input)?;

    fn reverse(program: &[usize], a: usize) -> Option<usize> {
        if program.is_empty() {
            return Some(a);
        }

        for b in 0..8 {
            let a = (a << 3) + b;
            let mut b = b ^ 2;
            let c = a >> b;
            b ^= 7;
            b ^= c;
            b &= 7;

            if b == program[program.len() - 1] {
                if let Some(a) = reverse(&program[..program.len() - 1], a) {
                    return Some(a);
                }
            }
        }

        None
    }

    let a = reverse(&program, 0).unwrap();

    Ok(a.to_string())
}
