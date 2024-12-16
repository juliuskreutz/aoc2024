mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

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

    run(args.day)
}

#[tracing::instrument]
fn run(day: Option<usize>) -> anyhow::Result<()> {
    let days = [
        day01::solve,
        day02::solve,
        day03::solve,
        day04::solve,
        day05::solve,
        day06::solve,
        day07::solve,
        day08::solve,
        day09::solve,
        day10::solve,
        day11::solve,
        day12::solve,
        day13::solve,
        day14::solve,
        day15::solve,
        day16::solve,
    ];

    match day {
        Some(i) if i - 1 < days.len() => days[i - 1](),
        _ => {
            for day in days {
                day()?;
            }
            Ok(())
        }
    }
}
