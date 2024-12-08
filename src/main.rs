mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();

    match args.get(1).and_then(|s| s.parse::<usize>().ok()) {
        Some(1) => day01::solve()?,
        Some(2) => day02::solve()?,
        Some(3) => day03::solve()?,
        Some(4) => day04::solve()?,
        Some(5) => day05::solve()?,
        Some(6) => day06::solve()?,
        Some(7) => day07::solve()?,
        Some(8) => day08::solve()?,
        _ => {
            day01::solve()?;
            day02::solve()?;
            day03::solve()?;
            day04::solve()?;
            day05::solve()?;
            day06::solve()?;
            day07::solve()?;
            day08::solve()?;
        }
    }

    Ok(())
}
