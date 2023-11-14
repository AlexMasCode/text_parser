use anyhow::{anyhow, Result};
use structopt::StructOpt;
use text_parser::text_parser;

#[derive(Debug, StructOpt)]
#[structopt(name = "text_parser", about = "Parse expressions, dates, words, numbers, and word_or_number")]
struct Cli {
    #[structopt(short = "e", long = "expression", help = "Expression to parse")]
    expression: Option<String>,

    #[structopt(short = "d", long = "date", help = "Date to parse")]
    date: Option<String>,

    #[structopt(short = "w", long = "word", help = "Word to parse")]
    word: Option<String>,

    #[structopt(short = "n", long = "number", help = "Number to parse")]
    number: Option<String>,

    #[structopt(short = "o", long = "word_or_number", help = "Word or number to parse")]
    word_or_number: Option<String>,
}

fn main() -> Result<()> {
    let cli = Cli::from_args();

    if let Some(expression) = cli.expression {
        match text_parser::expression(&expression) {
            Ok(parsed) => println!("Parsed expression: {:?}", parsed),
            Err(err) => eprintln!("Error parsing expression: {}", err),
        }
    }

    if let Some(date) = cli.date {
        match text_parser::date(&date) {
            Ok(parsed) => println!("Parsed date: {:?}", parsed),
            Err(err) => eprintln!("Error parsing date: {}", err),
        }
    }

    if let Some(word) = cli.word {
        match text_parser::word(&word) {
            Ok(parsed) => println!("Parsed word: {:?}", parsed),
            Err(err) => eprintln!("Error parsing word: {}", err),
        }
    }

    if let Some(number) = cli.number {
        match text_parser::number(&number) {
            Ok(parsed) => println!("Parsed number: {:?}", parsed),
            Err(err) => eprintln!("Error parsing number: {}", err),
        }
    }

    if let Some(word_or_number) = cli.word_or_number {
        match text_parser::word_or_number(&word_or_number) {
            Ok(parsed) => println!("Parsed word_or_number: {:?}", parsed),
            Err(err) => eprintln!("Error parsing word_or_number: {}", err),
        }
    }

    Ok(())
}
