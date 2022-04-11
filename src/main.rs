use std::io::{stdin,stdout,Write};
use std::{thread, time};

use clap::Parser;
use reqwest::header::USER_AGENT;
use serde::{Serialize, Deserialize};

/// Read and create comments for GitHub PR
#[derive(Parser)]
struct Cli {
    /// Id of a PR to work with
    pr_id: String,
    /// GitHub auth token
    github_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct PrComment {
    url: String,
    id: i64,
    created_at: String,
    body: String
}

struct UrlParts<'a> {
    base_url: String,
    owner: &'a str,
    repo: &'a str, 
    pr_id: &'a str,
    github_token: &'a str,
}

impl UrlParts<'_> {
    fn get_url(&self) -> String {
        self.base_url.to_owned()+self.owner+"/"+self.repo+"/issues/"+self.pr_id+"/comments"
    }
}


fn print_pr_comments(get_url: &String, client: &reqwest::blocking::Client) -> Result<(), Box<dyn std::error::Error>> {
        
    let res = client.get(get_url)
        .header(USER_AGENT, "Rust/reqwest")
        .send()?
        .text()?;
    
    let comments: Vec<PrComment> = serde_json::from_str(&res)?;

    for comment in comments.iter() {
        println!("{}", comment.body);
    }
    
    Ok(())
}

fn post_pr_comment(url_p: &UrlParts, client: &reqwest::blocking::Client, comment: &str) -> Result<(), Box<dyn std::error::Error>> {
    let post_body = "{\"body\":\"".to_owned()+comment+"\"}";

    client.post(url_p.get_url())
        .header(USER_AGENT, "Rust/reqwest")
        .body(post_body)
        .basic_auth(url_p.owner, Some(url_p.github_token))
        .send()?;

    Ok(())
    }

fn ask_for_comment() -> String {
    
    let mut s=String::new();
    print!("Please enter the comment text: ");
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Please, stick to UTF-8");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    return s
}

fn main() {
    let args = Cli::parse();
    let client = reqwest::blocking::Client::new();
    let up: UrlParts = UrlParts { 
        base_url: "https://api.github.com/repos/".to_string(), 
        owner: "dvoriak", 
        repo: "fizyr_interview", 
        pr_id: &*args.pr_id,
        github_token: &args.github_token,
    };
    let issues_url: String = up.get_url();

    loop {
        print_pr_comments(&issues_url, &client).ok();
        let comment_text: String = ask_for_comment();
        post_pr_comment(&up, &client, &comment_text).ok();
        // Unless there is a wait time GitHub api does not return the last created comment in get comments
        thread::sleep(time::Duration::from_secs(5));
    }
}
