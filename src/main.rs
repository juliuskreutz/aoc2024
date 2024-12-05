mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

fn main() -> anyhow::Result<()> {
    day01::solve()?;
    day02::solve()?;
    day03::solve()?;
    day04::solve()?;
    day05::solve()?;

    Ok(())
}
