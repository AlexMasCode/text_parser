pub use pest_derive::Parser;
pub use pest::Parser;

peg::parser! {
  pub grammar text_parser() for str {
    rule letter() -> char
            = ['a'..='z' | 'A'..='Z']

        rule digit() -> char
            = ['0'..='9']

        pub rule word() -> &'input str
            = s:$(['a'..='z' | 'A'..='Z']+) { s }

        pub rule number() -> &'input str
            = s:$(['0'..='9']+) { s }

        pub rule word_or_number() -> &'input str
            = s:$(['a'..='z' | 'A'..='Z']+ / ['0'..='9']+) { s }


      pub rule expression() -> Expression
            = left:number() "+" right:number() {
                Expression {
                    left: left.to_string(),
                    right: right.to_string(),
                    operation: Operation::Add,
                }
            }
            / left:number() "-" right:number() {
                Expression {
                    left: left.to_string(),
                    right: right.to_string(),
                    operation: Operation::Subtract,
                }
            }
            / left:number() "*" right:number() {
                Expression {
                    left: left.to_string(),
                    right: right.to_string(),
                    operation: Operation::Multiply,
                }
            }
            / left:number() "/" right:number() {
                Expression {
                    left: left.to_string(),
                    right: right.to_string(),
                    operation: Operation::Divide,
                }
            }      

      pub rule date() -> Date
          = y:number() "-" m:number() "-" d:number() {
              Date {
                  year: y.to_string(),
                  month: m.to_string(),
                  day: d.to_string(),
                  
              }
          }

      
  }
}

#[derive(Debug)]
pub struct Date {
  pub day: String,
  pub month: String,
  pub year: String,
}

#[derive(Debug, PartialEq)]
pub struct Expression {
    pub left: String,
    pub right: String,
    pub operation: Operation,
}

#[derive(Debug, PartialEq)]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}


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