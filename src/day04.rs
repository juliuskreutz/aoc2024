pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/04.txt")?;

    println!("Day04 Part1: {}", part1(&input)?);
    println!("Day04 Part2: {}", part2(&input)?);

    Ok(())
}

fn pad<T: Clone>(v: &mut Vec<T>, t: T, padding: usize) {
    for _ in 0..padding {
        v.insert(0, t.clone());
        v.push(t.clone());
    }
}

#[tracing::instrument(skip_all)]
pub fn part1(input: &str) -> anyhow::Result<String> {
    let padding = 3;

    let mut matrix: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    for row in &mut matrix {
        pad(row, '.', padding);
    }
    let len = matrix[0].len();
    pad(&mut matrix, vec!['.'; len], padding);

    let mut count = 0;
    for y in padding..matrix.len() - padding {
        for x in padding..matrix[0].len() - padding {
            for horizonal in [true, false] {
                let x1 = x;
                let x2 = x + if horizonal { 1 } else { 0 };
                let x3 = x + if horizonal { 2 } else { 0 };
                let x4 = x + if horizonal { 3 } else { 0 };

                let y1 = y;
                let y2 = y + if horizonal { 0 } else { 1 };
                let y3 = y + if horizonal { 0 } else { 2 };
                let y4 = y + if horizonal { 0 } else { 3 };

                let c1 = matrix[y1][x1];
                let c2 = matrix[y2][x2];
                let c3 = matrix[y3][x3];
                let c4 = matrix[y4][x4];

                if matches!(
                    [c1, c2, c3, c4],
                    ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X']
                ) {
                    count += 1;
                }
            }

            for down in [true, false] {
                let x1 = x;
                let x2 = x + 1;
                let x3 = x + 2;
                let x4 = x + 3;

                let y1 = y;
                let y2 = if down { y + 1 } else { y - 1 };
                let y3 = if down { y + 2 } else { y - 2 };
                let y4 = if down { y + 3 } else { y - 3 };

                let c1 = matrix[y1][x1];
                let c2 = matrix[y2][x2];
                let c3 = matrix[y3][x3];
                let c4 = matrix[y4][x4];

                if matches!(
                    [c1, c2, c3, c4],
                    ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X']
                ) {
                    count += 1;
                }
            }
        }
    }

    Ok(count.to_string())
}

#[tracing::instrument(skip_all)]
pub fn part2(input: &str) -> anyhow::Result<String> {
    let padding = 2;

    let mut matrix: Vec<Vec<_>> = input.lines().map(|l| l.chars().collect()).collect();

    for row in &mut matrix {
        pad(row, '.', padding);
    }
    let len = matrix[0].len();
    pad(&mut matrix, vec!['.'; len], padding);

    let mut count = 0;
    for y in padding..matrix.len() - padding {
        for x in padding..matrix[0].len() - padding {
            let c1 = matrix[y][x];
            let c2 = matrix[y + 1][x + 1];
            let c3 = matrix[y + 2][x + 2];

            let c4 = matrix[y][x + 2];
            let c5 = matrix[y + 1][x + 1];
            let c6 = matrix[y + 2][x];

            if matches!([c1, c2, c3], ['M', 'A', 'S'] | ['S', 'A', 'M'])
                && matches!([c4, c5, c6], ['M', 'A', 'S'] | ['S', 'A', 'M'])
            {
                count += 1;
            }
        }
    }

    Ok(count.to_string())
}
