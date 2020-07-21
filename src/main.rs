#[macro_use]
extern crate lazy_static;
use peg;
use rand::prelude::*;
use regex::Regex;
use std::env;

#[derive(Debug)]
pub enum RngArgs {
    Natural { min: u32, max: u32 },
}

peg::parser!( grammar monkeys() for str {
    rule number() -> u32
        = n:$(['0'..='9']+) { n.parse().unwrap() }
    rule args_natural() -> (u32, u32)
        = "(" min:number() "," max: number() ")" {(min, max)}
    pub rule gen_natural() -> RngArgs
        = "n" args:args_natural()? {
            let args = args.unwrap_or((0, u32::MAX));
            RngArgs::Natural{ min: args.0, max: args.1}
    }

});

fn random_natural(min: u32, max: u32) -> String {
    thread_rng().gen_range(min, max).to_string()
}

fn random_value(args: &RngArgs) -> String {
    match args {
        &RngArgs::Natural { min, max } => random_natural(min, max),
    }
}

struct ReplaceInstruction {
    position: (usize, usize),
    args: RngArgs,
}

struct Replacer {
    text: String,
    instructions: Vec<ReplaceInstruction>,
}

impl Replacer {
    fn new(text: String) -> Self {
        let instructions = Self::gen_instructions(&text);
        Self { text, instructions }
    }
    fn gen_instructions(text: &String) -> Vec<ReplaceInstruction> {
        lazy_static! {
            static ref RE: Regex = Regex::new("\\$\\{(?P<expression>\\S+)\\}").unwrap();
        }
        // Position to start looking for expression
        let mut start = 0;
        let mut instructions: Vec<ReplaceInstruction> = Vec::new();
        // Iterating over matching expressions until no more expression is found
        loop {
            let mut locs = RE.capture_locations();
            let cap_match = RE.captures_read_at(&mut locs, &text, start);
            if cap_match.is_none() { // pattern not found
                break;
            }
            // Position of the text to be replaced
            let outer_loc = locs.get(0).unwrap();
            // Position of the text with replacement expression
            let inner_loc = locs.get(1).unwrap();
            // replacement expression
            let expression = &text[inner_loc.0..inner_loc.1];

            start = outer_loc.1;
            let instruction = ReplaceInstruction {
                position: outer_loc,
                args: monkeys::gen_natural(&expression).unwrap(),
            };
            instructions.push(instruction);
        }
        instructions
    }

    fn replace(&self) -> String {
        let mut result = self.text.clone();
        for instruction in self.instructions.iter().rev() {
            let position = instruction.position.0..instruction.position.1;
            result.replace_range(position, &random_value(&instruction.args).to_string());
        }
        result
    }
}

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
