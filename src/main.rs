#[macro_use]
extern crate lazy_static;
use crate::replacer::Replacer;
use itertools::Itertools;
use rayon::{
    iter::{IntoParallelIterator, ParallelIterator},
    ThreadPoolBuilder,
};
use structopt::StructOpt;

pub mod generators;

mod lang;
mod replacer;

#[derive(Debug, StructOpt)]
#[structopt(name = "monkeys", about = "Templated random strings")]
struct Opt {
    template: String,

    #[structopt(default_value = "1")]
    repeats: u32,

    #[structopt(short, long)]
    jobs: Option<usize>,
}

fn main() {
    const CHUNK_SIZE: usize = 5000; // Number of lines calculated/printed together

    let opt = Opt::from_args();
    // set the number of concurrent jobs from user input,
    // let rayon decide, otherwise
    if let Some(jobs) = opt.jobs {
        ThreadPoolBuilder::new()
            .num_threads(jobs)
            .build_global()
            .unwrap();
    }
    let replacer = Replacer::new(opt.template.clone());
    let repeater = (0..opt.repeats).chunks(CHUNK_SIZE);
    for chunk in repeater.into_iter() {
        let data: Vec<_> = chunk
            .collect::<Vec<_>>()
            .into_par_iter()
            .map(|order| replacer.replace(order))
            .collect();
        println!("{}", data.join("\n"));
    }
}
