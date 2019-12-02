extern crate reqwest;
extern crate tempfile;

use std::io::{copy, Write};
use std::fs::File;
use tempfile::Builder;
use std::collections::HashMap;
use reqwest::cookie::Cookie;
use reqwest::header::HeaderValue;
use std::fs;
use std::path::Path;
use std::env;

fn get_body(problem: i32) -> String {

    fs::create_dir_all("./cache").expect("");

    let filepath = format!("./cache/problem-{}", problem);
    let url = format!("https://adventofcode.com/2019/day/{}/input", problem);

    if Path::new(&filepath).exists() {
        fs::read_to_string(filepath).unwrap()
    } else {

        let session_token = match env::var("SESSION") {
            Ok(token) => token,
            Err(e) => panic!("No session token"),
        };

        let session_token = format!("session={};", session_token);

        let client = reqwest::Client::new();
        let body = client
            .get(&url)
            .header("Cookie", session_token)
            .send().expect("").text().expect("");

        let mut f = File::create(format!("./cache/problem-{}", problem)).expect("");
        f.write_all(body.as_bytes());

        body
    }
}

fn parse_body(body: String) -> Vec<i32> {

    let a : Vec<i32> = body.
        split("\n")
        .filter_map(|s| {
            //map(|s| {

            let s = s.trim();
            if !s.is_empty() {
                Some(s.parse::<i32>().unwrap())
            } else {
                None
            }
        }).collect();

    a
}

pub fn get_problem(problem: i32) -> Vec<i32> {
    parse_body(get_body(problem))
}