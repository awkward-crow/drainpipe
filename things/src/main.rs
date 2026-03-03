// things.rs

use regex::Regex;
use std::{collections::HashMap, fs::File, io::Read};

struct DataStorageManager {
    data: String,
}

impl DataStorageManager {
    fn new(filename: &str) -> Self {
        let mut data = String::new();
        {
            let mut file = File::open(filename).unwrap();
            file.read_to_string(&mut data).unwrap();
        }
        let re = Regex::new(r"[\W_]+").unwrap();
        data = re.replace_all(&data, " ").to_lowercase();
        DataStorageManager { data }
    }

    fn words(&self) -> Vec<String> {
        self.data
            .split_whitespace()
            .map(str::to_string)
            .collect::<Vec<_>>()
    }
}

struct StopWordManager {
    stop_words: Vec<String>,
}

impl StopWordManager {
    fn new() -> Self {
        let mut s = String::new();
        {
            let mut file = File::open("./stop_words.txt").unwrap();
            file.read_to_string(&mut s).unwrap();
        }
        let mut t: Vec<_> = s.split(",").map(str::to_string).collect();
        for c in 'a'..='z' {
            t.push(c.to_string());
        }
        StopWordManager { stop_words: t }
    }

    fn is_stop_word(&self, w: &String) -> bool {
        self.stop_words.contains(w)
    }

    fn _show(&self) {
        println!("{:?}", self.stop_words);
    }
}

struct WordCountManager {
    counts: HashMap<String, i32>,
}

impl WordCountManager {
    fn new() -> Self {
        WordCountManager {
            counts: HashMap::new(),
        }
    }

    fn increment(&mut self, w: String) -> i32 {
        let count = self.counts.entry(w).or_insert(0);
        *count += 1;
        *count
    }

    fn most_common(&self, n: usize) -> Vec<(&String, &i32)> {
        let mut v = self.counts.iter().collect::<Vec<_>>();
        v.sort_by(|p, q| q.1.cmp(p.1));
        v.into_iter().take(n).collect()
    }

    fn _more_common(&self, n: usize) -> Vec<(String, i32)> {
        let mut v = self.counts.iter().collect::<Vec<_>>();
        v.sort_by(|p, q| q.1.cmp(p.1));
        let mut r = Vec::new();
        for (w, k) in v.into_iter().take(n) {
            r.push((String::from(w), *k));
        }
        r
    }

    fn _show(&self) {
        println!("{:?}", self.counts);
    }
}

struct WordFrequencyController {
    data_storage_manager: DataStorageManager,
    stop_word_manager: StopWordManager,
    word_count_manager: WordCountManager,
}

impl WordFrequencyController {
    fn new(filename: &str) -> Self {
        WordFrequencyController {
            data_storage_manager: DataStorageManager::new(filename),
            stop_word_manager: StopWordManager::new(),
            word_count_manager: WordCountManager::new(),
        }
    }

    fn run(&mut self) {
        for u in self.data_storage_manager.words() {
            if !self.stop_word_manager.is_stop_word(&u) {
                self.word_count_manager.increment(u);
            }
        }
        for (k, n) in self.word_count_manager.most_common(25) {
            println!("{} - {}", k, n);
        }
    }
}

fn main() {
    let filename = "pride-and-prejudice.txt";
    let mut controller = WordFrequencyController::new(filename);

    controller.run();
}

// end
