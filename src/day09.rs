pub fn solve() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("input/09.txt")?;

    println!("Day09 Part1: {}", part1(&input)?);
    println!("Day09 Part2: {}", part2(&input)?);

    Ok(())
}

pub fn part1(input: &str) -> anyhow::Result<String> {
    let numbers: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut fs = Vec::new();

    for (i, &n) in numbers.iter().enumerate() {
        let file = if i % 2 == 0 { Some(i / 2) } else { None };

        for _ in 0..n {
            fs.push(file);
        }
    }

    let mut i = 0;
    let mut j = fs.len() - 1;

    loop {
        while fs[j].is_none() {
            j -= 1;
        }

        while fs[i].is_some() {
            i += 1;
        }

        if i >= j {
            break;
        }

        fs.swap(i, j);
    }

    let mut sum = 0;

    for (i, n) in fs.into_iter().enumerate() {
        let Some(n) = n else {
            break;
        };

        sum += i * n;
    }

    Ok(sum.to_string())
}

pub fn part2(input: &str) -> anyhow::Result<String> {
    let numbers: Vec<_> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut fs = Vec::new();

    let mut file_positions = Vec::new();
    let mut empty_positions = Vec::new();

    for (i, &n) in numbers.iter().enumerate() {
        let file = if i % 2 == 0 {
            file_positions.push((fs.len(), n));

            Some(i / 2)
        } else {
            empty_positions.push((fs.len(), n));

            None
        };

        for _ in 0..n {
            fs.push(file);
        }
    }

    for &(file_position, file_amount) in file_positions.iter().rev() {
        let mut empty_position_index = None;

        for (i, &(empty_position, empty_amount)) in empty_positions.iter().enumerate() {
            if empty_position >= file_position {
                break;
            }

            if empty_amount >= file_amount {
                empty_position_index = Some((i, empty_position));
                break;
            }
        }

        let Some((i, empty_position)) = empty_position_index else {
            continue;
        };

        for i in 0..file_amount {
            fs.swap(file_position + i, empty_position + i);
        }

        empty_positions[i].0 += file_amount;
        empty_positions[i].1 -= file_amount;
    }

    let mut sum = 0;

    for (i, &n) in fs.iter().enumerate() {
        if let Some(n) = n {
            sum += i * n;
        }
    }

    Ok(sum.to_string())
}
