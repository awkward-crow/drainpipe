// pipeline.rs

use regex::Regex;
use std::{collections::HashMap, fs::File, io::Read};

fn read_file(filename: &str) -> String {
    let mut data = String::new();
    {
        let mut file = File::open(filename).unwrap();
        file.read_to_string(&mut data).unwrap();
    }
    data
}

fn filter_chars_and_normalize(data: String) -> String {
    let re = Regex::new(r"[\W_]+").unwrap();
    re.replace_all(&data, " ").to_lowercase()
}

fn scan(data: &str) -> impl Iterator<Item = String> + use<'_> {
    data.split_whitespace().map(str::to_string)
}

fn remove_stop_words<T: Iterator<Item = String>>(words: T) -> impl Iterator<Item = String> {
    let mut s = String::new();
    {
        let mut file = File::open("./stop_words.txt").unwrap();
        file.read_to_string(&mut s).unwrap();
    }
    let mut t: Vec<_> = s.split(",").map(str::to_string).collect();
    for c in 'a'..='z' {
        t.push(c.to_string());
    }
    words.filter(move |w| !t.contains(w))
}

fn counts<T: Iterator<Item = String>>(words: T) -> HashMap<String, i32> {
    let mut h = HashMap::new();
    for w in words {
        let count = h.entry(w).or_insert(0);
        *count += 1;
    }
    h
}

fn sort(counts: HashMap<String, i32>) -> Vec<(String, i32)> {
    let mut v = counts.into_iter().collect::<Vec<_>>();
    v.sort_by(|p, q| q.1.cmp(&p.1));
    v
}

fn main() {
    let filename = "pride-and-prejudice.txt";
    let z = sort(counts(remove_stop_words(scan(
        &filter_chars_and_normalize(read_file(filename)),
    ))));

    for (k, n) in &z[0..25] {
        println!("{} - {}", k, n);
    }
}

// end
