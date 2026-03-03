# drainpipe

## latest

 - bin/norse.rs, persistent tables style using polars
 - bin/things.rs
 - continuation passing is doomed

## next steps

 - pass file name on command line
 - duckdb, postgres
 - web or http api, rocket?

 - tidy up pipeline ??

## usage

Try,

```sh
cargo run --bin drainpipe
```

or replace `drainpipe` with e.g. `norse`.

## todo

 - find alternative to `.get_row` in polars, see bin/norse.rs
 - counts as `reduce`: apply `reduce` to iterator with closure over the hashmap?
 - try pipeline with iterators over &str, this would require sorting out lifetimes!

## borrowing collections - or `.into_iter()`

what happens to say a hashmap when it is passed by reference (is this the rust jargon) to a function e.g.

```rust
let mut h = HashMap<String, i32>::new();
<< data is added to h, then >>
print_em(&h);
```

see `main.rs` and/or `bin/pipeline.rs`. it seems that `&h` has type `&HashMap<&String, &i32>`, in other words, 
the keys and values in the map become `&_`. is this right?

Or, is it to do with `.into_iter()`?

## style 'kick-forward' i.e. continuation passing

the signature of the continuation must (?) reach all the way to the end of the computation!? this is not practical!!

## style 'the one' or 'monadic identity'

this proved too difficult, rust is not that flexible -- revisit?

## style 'golf'

```rust
let s = fs::read_to_string("./stop_words.txt").expect("can't find stop_words.txt?");
let ascii = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x",  "y", "z"];
let stop_words = s.split(",").chain(ascii).collect::<Vec<&str>>();
```

## style 'pipeline'

this uses iterators over or vectors of `String`. the more monolithic (i.e. one lifetime?) version in main.rs can use `&str`, the references are good within the single scope -- try to understand this better!

## read a file

What is the effect of the block around `let mut file ... `?

```rust
use std::{fs::File, io::Read};

let filename = "pride-and-prejudice.txt";
let mut data = String::new();
{
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut data).unwrap();
}
```

Use of `.expect` ??


### end
