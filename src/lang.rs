use crate::generators::GenArgs;
use peg;



peg::parser!( grammar monkeys() for str {
    rule number() -> u32
        = n:$(['0'..='9']+) { n.parse().unwrap() }
    rule args_natural() -> (u32, u32)
        = "(" min:number() "," max: number() ")" {(min, max)}
    pub rule gen_natural() -> GenArgs
        = "n" args:args_natural()? {
            let args = args.unwrap_or((0, u32::MAX));
            GenArgs::Natural{ min: args.0, max: args.1}
    }

});

type ParsingResult = Result<GenArgs,peg::error::ParseError<peg::str::LineCol>>;

pub fn parse(expression: &str) -> ParsingResult {
    monkeys::gen_natural(expression)
}
