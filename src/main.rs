use advent_of_rust::Solutions;
use advent_of_rust::template;
use anyhow::{Context, Result};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "aor")]
#[command(about = "Advent of Rust solution runner and scaffolder", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Solve { year: u16, day: u8 },
    List { year: Option<u16> },
    New { year: u16, day: u8 },
}

fn run_solver(year: u16, day: u8) -> Result<()> {
    if !(1..=25).contains(&day) {
        anyhow::bail!(format!("Day must be between 1 & 25, not {}", day));
    }

    let solver = Solutions::get_solver(year, day).with_context(|| {
        format!(
            "Crash while accessing solution for year {} and day {}",
            year, day
        )
    })?;
    solver()
}

fn list_solutions(year: Option<u16>) -> Result<()> {
    let years_to_list = if let Some(year_filt) = year {
        vec![year_filt]
    } else {
        Solutions::list_years()
    };

    for y in years_to_list {
        let days = Solutions::available_days(y);
        if days.is_empty() {
            continue;
        }

        println!("Year {}", y);
        println!("{}", "-".repeat(20));

        let mut day_range = vec![];
        for &day in &days {
            day_range.push(format!("{}", day));
        }

        println!("Days: {}", day_range.join(", "));
        println!("\n");
    }
    Ok(())
}

fn scaffold_day(year: u16, day: u8) -> Result<()> {
    if !(1..=25).contains(&day) {
        anyhow::bail!("Day must be between 1 and 25");
    }
    template::create_day_file(year, day)?;

    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Solve { year, day } => run_solver(year, day),
        Commands::List { year } => list_solutions(year),
        Commands::New { year, day } => scaffold_day(year, day),
    }
}
