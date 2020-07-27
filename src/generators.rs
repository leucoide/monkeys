use rand::prelude::*;

#[derive(Debug)]
pub enum GenArgs {
    Natural { min: u32, max: u32 },
    Float { min: f32, max: f32, round: usize },
}

fn random_natural(min: u32, max: u32) -> String {
    thread_rng().gen_range(min, max).to_string()
}

fn random_float(min: f32, max: f32, round: usize) -> String {
    let number: f32 = thread_rng().gen_range(min, max);
    format!("{:.1$}", number, round)
}

pub fn gen_value(args: &GenArgs) -> String {
    match args {
        &GenArgs::Natural { min, max } => random_natural(min, max),
        &GenArgs::Float { min, max, round } => random_float(min, max, round),
    }
}
