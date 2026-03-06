// norse.rs -- persistent tables using polars

use regex::Regex;
use std::fs;
use std::env;

use polars::chunked_array::ops::SortMultipleOptions;
use polars::frame::{column::Column, DataFrame};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filename = env::args().nth(1).expect("usage: norse <filename>");

    let s = fs::read_to_string("./stop_words.txt").expect("this should be ok");
    let stop_words = s.split(",").collect::<Vec<&str>>();

    let data = fs::read_to_string(&filename)
        .unwrap_or_else(|e| panic!("can't read {filename}: {e}"))
        .to_lowercase();

    let re = Regex::new(r"[a-z]{2,}").unwrap();

    let words = re
        .find_iter(&data)
        .map(|m| m.as_str())
        .filter(|m| !stop_words.contains(m))
        .collect::<Vec<_>>();

    let s = Column::new("words".into(), words);
    let df = DataFrame::new(vec![s])?
        .group_by(["words"])?
        .select(["words"])
        .count()?;
    let ordered = df.sort(
        ["words_count"],
        SortMultipleOptions::new().with_order_descending(true),
    )?;

    let word_col = ordered.column("words")?.str()?;
    let count_col = ordered.column("words_count")?.u32()?;

    for i in 0..25 {
        if let (Some(k), Some(n)) = (word_col.get(i), count_col.get(i)) {
            println!("{} - {}", k, n);
        }
    }

    Ok(())
}

// end
