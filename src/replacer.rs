use regex::Regex;
use crate::{generators,lang};

struct ReplaceInstruction {
    position: (usize, usize),
    args: generators::GenArgs,
}

pub struct Replacer {
    text: String,
    instructions: Vec<ReplaceInstruction>,
}

impl Replacer {
    pub fn new(text: String) -> Self {
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
                args: lang::parse(&expression).unwrap(),
            };
            instructions.push(instruction);
        }
        instructions
    }

    pub fn replace(&self) -> String {
        let mut result = self.text.clone();
        for instruction in self.instructions.iter().rev() {
            let position = instruction.position.0..instruction.position.1;
            result.replace_range(position, &generators::gen_value(&instruction.args));
        }
        result
    }
}
