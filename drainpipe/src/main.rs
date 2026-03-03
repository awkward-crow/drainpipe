// main.rs

use std::{collections::HashMap, env, fs, fs::File, io::Read};

use regex::Regex;

fn main() {
    let s = fs::read_to_string("./stop_words.txt").expect("this should be ok");
    let mut stop_words = s.split(",").collect::<Vec<&str>>();
    let ascii = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r",
        "s", "t", "u", "v", "w", "x", "y", "z",
    ];
    stop_words.extend(ascii);

    let filename = env::args().nth(1).expect("usage: drainpipe <filename>");
    let mut data = String::new();
    {
        let mut file = File::open(filename).unwrap();
        file.read_to_string(&mut data).unwrap();
    }

    let re = Regex::new(r"[\W_]+").unwrap();
    let words = re.replace_all(&data, " ").to_lowercase();

    let mut h = HashMap::new();
    for w in words.split_whitespace().filter(|w| !stop_words.contains(w)) {
        let count = h.entry(w).or_insert(0);
        *count += 1;
    }

    let mut v = h.into_iter().collect::<Vec<_>>();
    v.sort_by(|p, q| q.1.cmp(&p.1));

    println!();
    for (k, n) in &v[0..25] {
        println!("{} - {}", k, n);
    }

    println!("\nDoukipudonktan!\n");
}

// end
