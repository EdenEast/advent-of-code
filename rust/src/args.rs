use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "advent_of_code")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[command(visible_alias("r"), about = "Run a solution to a problem")]
    Run {
        day: u32,
        part: char,
        year: Option<u32>,
    },
    #[command(visible_alias("t"), about = "Run the test input to a problem")]
    Test {
        day: u32,
        part: char,
        year: Option<u32>,
    },
    #[command(visible_alias("l"), about = "List all solutions for a given year")]
    List { year: Option<u32> },
}
