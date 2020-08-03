#[macro_use]
extern crate lazy_static;
use crate::replacer::Replacer;
//use std::env;
use structopt::StructOpt;
pub mod generators;

mod lang;
mod replacer;

#[derive(Debug, StructOpt)]
#[structopt(name = "monkeys", about = "Templated random strings")]
struct Opt {
    // #[structopt()]
    template: String,
    #[structopt(default_value = "1")]
    repeats: u32,
}

fn main() {
    const CHUNK_SIZE: usize = 5000; // Number of lines calculated/printed together

    let opt = Opt::from_args();
    let replacer = Replacer::new(opt.template.clone());
    // iterating in chunks to minimize I/O overhead
    let mut range = (1..=opt.repeats).peekable();
    while range.peek().is_some() {
        let outputs: Vec<String> = range
            .by_ref()
            .take(CHUNK_SIZE)
            .map(|_| replacer.replace())
            .collect();
        println!("{}", outputs.join("\n"));
    }
}
