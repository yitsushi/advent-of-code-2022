pub mod day;
pub mod part;

use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub day: day::Day,
    #[arg(long)]
    pub part: part::Part,
}
