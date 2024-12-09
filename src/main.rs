mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

use clap::Parser;

#[derive(Parser)]
#[command()]
struct Args {
    #[arg()]
    day: Option<usize>,

    #[arg(short, long)]
    trace: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.trace {
        tracing_subscriber::fmt()
            .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            .init();
    }

    match args.day {
        Some(1) => day01::solve()?,
        Some(2) => day02::solve()?,
        Some(3) => day03::solve()?,
        Some(4) => day04::solve()?,
        Some(5) => day05::solve()?,
        Some(6) => day06::solve()?,
        Some(7) => day07::solve()?,
        Some(8) => day08::solve()?,
        Some(9) => day09::solve()?,
        _ => {
            day01::solve()?;
            day02::solve()?;
            day03::solve()?;
            day04::solve()?;
            day05::solve()?;
            day06::solve()?;
            day07::solve()?;
            day08::solve()?;
            day09::solve()?;
        }
    }

    Ok(())
}
