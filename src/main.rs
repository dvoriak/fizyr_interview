#![allow(unused)]
use clap::Parser;
use reqwest;

/// Read and create comments for GitHub PR
#[derive(Parser)]
struct Cli {
    /// Id of a PR to work with
    pr_id: String,
}

fn get_pr_comments(pr_id: String, client: reqwest::Client) {
    let get_url = "https://api.github.com/repos/dvoriak/fizyr_interview/pulls/comments";
    let res = client.get(get_url).await?.text().await?;

    return res
}

fn main() {
    let args = Cli::parse();
    let client = reqwest::Client::new();

    println!("body = {:?}", get_pr_comments(args.pr_id, client))
}
