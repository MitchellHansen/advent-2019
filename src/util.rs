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

fn get_body(problem: i32) -> String {

    fs::create_dir("./cache").expect("");

    let filepath = format!("./cache/problem-{}", problem);
    let url = format!("https://adventofcode.com/2019/day/{}/input", problem);

    if Path::new(&filepath).exists() {
        fs::read_to_string(filepath).unwrap()
    } else {
        let client = reqwest::Client::new();
        let body = client
            .get(&url)
            .header("Cookie", "session=53616c7465645f5f9a1fd404211ee5731f6b070241a384279cd0ac5ea569fb1c5ba9f009f866c493e23670d4042b1167;")
            .send().expect("").text().expect("");

        let mut f = File::create(format!("./problem-{}", problem)).expect("");
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