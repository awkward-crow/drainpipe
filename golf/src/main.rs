// golf.rs

// # Best wishes, Peter Norvig
// import re, sys, collections
//
// stopwords = set(open('../stop_words.txt').read().split(','))
// words = re.findall('[a-z]{2,}', open(sys.argv[1]).read().lower())
// counts = collections.Counter(w for w in words if w not in stopwords)
// for (w, c) in counts.most_common(25):
//     print(w, '-', c)

use regex::Regex;
use std::{collections::HashMap, fs}; //, fs::File, io::Read, collections::HashMap};

fn main() {
    let s = fs::read_to_string("./stop_words.txt").expect("this should be ok");
    let stop_words = s.split(",").collect::<Vec<&str>>();

    let data = fs::read_to_string("./pride-and-prejudice.txt")
        .expect("can't find p&p?")
        .to_lowercase();

    let re = Regex::new(r"[a-z]{2,}").unwrap();

    let mut h = HashMap::new();
    for w in re
        .find_iter(&data)
        .map(|m| m.as_str())
        .filter(|m| !stop_words.contains(m))
    {
        *h.entry(w).or_insert(0) += 1;
    }

    let mut v = h.into_iter().collect::<Vec<_>>();
    v.sort_by(|p, q| q.1.cmp(&p.1));

    for (k, n) in &v[0..25] {
        println!("{} - {}", k, n);
    }
}

// end
