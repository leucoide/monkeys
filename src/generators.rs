use rand::prelude::*;

#[derive(Debug)]
pub enum GenArgs {
    Natural { min: u32, max: u32 },
}

fn random_natural(min: u32, max: u32) -> String {
    thread_rng().gen_range(min, max).to_string()
}

pub fn gen_value(args: &GenArgs) -> String {
    match args {
        &GenArgs::Natural { min, max } => random_natural(min, max),
    }
}
