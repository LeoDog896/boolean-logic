use clap::Parser as ClapParser;
use pest::Parser as PestParser;

extern crate pest;

#[derive(pest_derive::Parser)]
#[grammar = "boolean.pest"]
struct BooleanParser;

/// Analyze a passed in boolean expression
#[derive(clap::Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The expression to analyze
    expression: String,
}

fn main() {
    let args = Args::parse();
    let expression = args.expression;

    let pairs = BooleanParser::parse(Rule::calculation, &expression)
        .expect("unsuccessful parse")
        .next().expect("no child pair?");

    match pairs.as_rule() {
        Rule::expr => {
            for pair in pairs.into_inner() {
                println!("Rule: {:?}", pair.as_rule());
            }
        },
        Rule::EOI => (),
        _ => unreachable!(),
    }
}
