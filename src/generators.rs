use rand::prelude::*;

#[derive(Debug)]
pub enum GenArgs {
    Natural { min: u32, max: u32 },
    Float { min: f32, max: f32, round: usize },
    SeqNatural { start: u32, step: u32 },
}

fn random_natural(min: u32, max: u32) -> String {
    thread_rng().gen_range(min, max).to_string()
}

fn random_float(min: f32, max: f32, round: usize) -> String {
    let number: f32 = thread_rng().gen_range(min, max);
    format!("{:.1$}", number, round)
}

fn sequential_natural(start: u32, step: u32, order: u32) -> String {
    (start + order * step).to_string()
}

pub fn gen_value(args: &GenArgs, order: u32) -> String {
    match args {
        &GenArgs::Natural { min, max } => random_natural(min, max),
        &GenArgs::Float { min, max, round } => random_float(min, max, round),
        &GenArgs::SeqNatural { start, step } => sequential_natural(start, step, order),
    }
}
