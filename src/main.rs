#![allow(unused)]
use clap::Parser;

/// Read and create comments for GitHub PR
#[derive(Parser)]
struct Cli {
    /// Id of a PR to work with
    pr_id: String,
}

fn main() {
    let args = Cli::parse();
}
