// infinite_mirror.rs

use regex::Regex;
use std::{cmp::min, collections::HashMap, fs};

fn count(words: &[String], stop_words: &Vec<String>, counts: &mut HashMap<String, i32>) {
    if !words.is_empty() {
        let w = words[0].clone();
        if !stop_words.contains(&w) {
            *counts.entry(w).or_insert(0) += 1;
        }
        count(&words[1..], stop_words, counts);
    }
}

fn wf_print(counts: &[(String, i32)]) {
    if !counts.is_empty() {
        let (k, n) = counts[0].clone();
        println!("{} - {}", k, n);
        wf_print(&counts[1..]);
    }
}

fn main() {
    let s = fs::read_to_string("./stop_words.txt").expect("can't find stop_words.txt");
    let stop_words: Vec<_> = s.split(",").map(str::to_string).collect();

    let data = fs::read_to_string("./pride-and-prejudice.txt")
        .expect("can't find p&p?")
        .to_lowercase();
    let re = Regex::new(r"[a-z]{2,}").unwrap();
    let words: Vec<_> = re
        .find_iter(&data)
        .map(|m| m.as_str())
        .map(str::to_string)
        .collect();

    let n = words.len();
    let block = 15000;
    let mut counts: HashMap<String, i32> = HashMap::new();
    for i in (0..n).step_by(block) {
        count(&words[i..min(i + block, n)], &stop_words, &mut counts);
    }

    let mut v = counts.into_iter().collect::<Vec<_>>();
    v.sort_by(|p, q| q.1.cmp(&p.1));
    wf_print(&v[0..25]);
}

// end
