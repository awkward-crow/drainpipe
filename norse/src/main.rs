// norse.rs -- persistent tables using polars

use regex::Regex;
use std::fs;

use polars::chunked_array::ops::SortMultipleOptions;
use polars::datatypes::AnyValue::{String, UInt32};
use polars::frame::{column::Column, row::Row, DataFrame};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let s = fs::read_to_string("./stop_words.txt").expect("this should be ok");
    let stop_words = s.split(",").collect::<Vec<&str>>();

    let data = fs::read_to_string("./pride-and-prejudice.txt")
        .expect("can't find p&p?")
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

    for i in 0..25 {
        if let Ok(Row(r)) = ordered.get_row(i) {
            if let [String(k), UInt32(n)] = r.as_slice() {
                println!("{} - {}", k, n);
            }
        }
    }

    Ok(())
}

// end
