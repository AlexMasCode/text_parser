# Text Parser

This Rust library provides parsing functionality for expressions, dates, words, numbers, and word_or_number using the `peg` library.

## Parsing Process

The `text_parser` library is designed to parse and validate various inputs. It defines a grammar in the `lib.rs` file, specifying the format of valid expressions, dates, words, numbers, and word_or_number. The parser then uses this grammar to validate input strings provided by the user.

The parsing process involves the following steps:

1. The input string is processed by the `text_parser`, which checks if it matches the defined grammar rules for expressions, dates, words, numbers, or word_or_number.

2. The parser validates the input according to the specified rules and returns the parsed result.

## Features

- Parse mathematical expressions with basic operations (+, -, *, /).
- Validate and parse date strings in the format "YYYY-MM-DD."
- Parse individual words and numbers.
- Parse strings that can be either words or numbers.

## Usage

You can integrate the `text_parser` into your Rust project to parse expressions, dates, words, numbers, and word_or_number. The library provides separate functions for each type of input.

## Getting Started

Clone the repository and explore the examples in the `main.rs` file to understand how to use the library in your project.

## Command-Line Options
The program accepts the following command-line options:

-e or --expression: Expression to parse
-d or --date: Date to parse
-w or --word: Word to parse
-n or --number: Number to parse
-o or --word_or_number: Word or number to pars

### Example

# Parse an expression
cargo run --release -- -e "2+3"

# Parse a date
cargo run --release -- -d "2022-11-13"

# Parse a word
cargo run --release -- -w "hello"

# Parse a number
cargo run --release -- -n "42"

# Parse a word or number
cargo run --release -- -o "world"

### Tests

All tests can be tested by the command:
cargo test

```rust

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_addition() {
        assert_eq!(
            text_parser::expression("2+3"),
            Ok(Expression {
                left: "2".to_string(),
                right: "3".to_string(),
                operation: Operation::Add,
            })
        );
    }

    #[test]
    fn test_subtraction() {
        assert_eq!(
            text_parser::expression("5-2"),
            Ok(Expression {
                left: "5".to_string(),
                right: "2".to_string(),
                operation: Operation::Subtract,
            })
        );
    }

    #[test]
    fn test_multiplication() {
        assert_eq!(
            text_parser::expression("4*6"),
            Ok(Expression {
                left: "4".to_string(),
                right: "6".to_string(),
                operation: Operation::Multiply,
            })
        );
    }

    #[test]
    fn test_division() {
        assert_eq!(
            text_parser::expression("8/2"),
            Ok(Expression {
                left: "8".to_string(),
                right: "2".to_string(),
                operation: Operation::Divide,
            })
        );
    }

    #[test]
    fn test_invalid_expression() {
        let result = text_parser::expression("invalid+expression");
        assert!(result.is_err());
    }
    
    #[test]
    fn test_parse_valid_date() {
        let parsed_date = text_parser::date("2022-11-13").unwrap();

        assert_eq!(parsed_date.day, "13");
        assert_eq!(parsed_date.month, "11");
        assert_eq!(parsed_date.year, "2022");
    }

    #[test]
    fn test_parse_invalid_date() {
        assert!(text_parser::date("2022-11-13-15").is_err());
    }

    #[test]
    fn test_parse_valid_date2() {
        assert!(text_parser::date("2022-11-13").is_ok());
    }

    #[test]
    fn test_valid_words() {
        assert_eq!(text_parser::word("hello"), Ok("hello"));
        assert_eq!(text_parser::word("World"), Ok("World"));
    }

    #[test]
    fn test_valid_numbers() {
        assert_eq!(text_parser::number("123"), Ok("123"));
        assert_eq!(text_parser::number("42"), Ok("42"));
    }

    #[test]
    fn test_valid_word_or_number() {
        assert_eq!(text_parser::word_or_number("hello"), Ok("hello"));
        assert_eq!(text_parser::word_or_number("123"), Ok("123"));
    }

    #[test]
    fn test_invalid_word_or_number() {
        assert!(text_parser::word_or_number("").is_err());
        assert!(text_parser::word_or_number("!@#").is_err());
    }
}
