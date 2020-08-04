use crate::generators::GenArgs;
use peg;

const DEFAULT_MIN_NATURAL: u32 = 0;
const DEFAULT_MAX_NATURAL: u32 = u32::MAX;

const DEFAULT_MIN_FLOAT: f32 = 0f32;
const DEFAULT_MAX_FLOAT: f32 = u32::MAX as f32;
const DEFAULT_ROUND_FLOAT: usize = 2;

const DEFAULT_SEQ_STEP: u32 = 1;
const DEFAULT_SEQ_START: u32 = 1;
const DEFAULT_SEQ_XP_START: u32 = 0;
peg::parser!( grammar monkeys() for str {
    // Literal number (used in arguments)
    rule number() -> u32
        = n:$(['0'..='9']+) { n.parse().unwrap() }
    rule _() -> ()
        = " "

    // Natural nubers generation
    rule args_natural() -> (u32, u32)
        = "(" min:number() "," " "* max: number() ")" {(min, max)}

    pub rule gen_natural() -> GenArgs
        = "n" args:args_natural()? {
            let args = args.unwrap_or((DEFAULT_MIN_NATURAL, DEFAULT_MAX_NATURAL));
            GenArgs::Natural{ min: args.0, max: args.1}
    }

    // Floating point number generation
    rule args_float() -> (u32, u32, u32)
        = "(" min:number() "," _* max: number() "," _* round:number()  ")" {(min, max, round)}
    pub rule gen_float() -> GenArgs
        = "f"  args:args_float()? {
            let args = args.unwrap_or((DEFAULT_MIN_FLOAT as u32, DEFAULT_MAX_FLOAT as u32, DEFAULT_ROUND_FLOAT as u32));
            GenArgs::Float{ min: args.0 as f32, max: args.1 as f32, round: args.2 as usize}
        }

    // Sequential numbers generation
    rule args_seq() -> (u32, u32)
        = "(" step:number()? "n" _* "+"? _* start:number()?  ")" {
            (start.unwrap_or(DEFAULT_SEQ_XP_START), step.unwrap_or(DEFAULT_SEQ_STEP))
        }

    pub rule gen_seq() -> GenArgs
        = "seq" args:args_seq()? {
            let args = args.unwrap_or((DEFAULT_SEQ_START, DEFAULT_SEQ_STEP));
            GenArgs::SeqNatural {start: args.0, step: args.1}
        }

    pub rule gen_args() -> GenArgs
        = gen_float() / gen_natural() / gen_seq()
});

type ParsingResult = Result<GenArgs, peg::error::ParseError<peg::str::LineCol>>;

pub fn parse(expression: &str) -> ParsingResult {
    monkeys::gen_args(expression)
}
