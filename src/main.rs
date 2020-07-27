#[macro_use]
extern crate lazy_static;
use std::env;
use crate::replacer::Replacer;
pub mod generators;
mod lang;
mod replacer;

fn main() {
    const CHUNK_SIZE: usize = 5000; //
    let args: Vec<String> = env::args().collect();
    let template = &args[1];
    let replacer = Replacer::new(template.clone());
    let repeats: u32 = if args.len() > 2 {
        args[2].parse().unwrap()
    } else {
        1
    };

    // iterating in chunks to minimize I/O overhead
    let mut range = (1..=repeats).peekable();
    while range.peek().is_some() {
        let outputs: Vec<String> = range
            .by_ref()
            .take(CHUNK_SIZE)
            .map(|_| replacer.replace())
            .collect();
        println!("{}", outputs.join("\n"));
    }
}
