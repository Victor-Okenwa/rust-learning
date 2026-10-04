use std::collections::HashMap;

// ============================================================
// JsonValue: the "target" type — what a parsed JSON becomes
// ============================================================
// JSON has exactly six kinds of values. Each variant of this
// enum represents one kind, and carries the data that kind holds.

#[derive(Debug)]
enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

// ============================================================
// Token: the output of the tokenizer — flat pieces of the input
// ============================================================
// A JSON string is broken into these small units. The parser
// will consume tokens one at a time to build a JsonValue.
//
// `Eof` is a sentinel meaning "end of input." Real JSON doesn't
// have it, but it makes parsing cleaner — the parser can always
// peek at the next token, even at the end.

#[derive(Debug)]
enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    String(String),
    Number(f64),
    Bool(bool),
    Null,
    Eof,
}

// ============================================================
// JsonError: what can go wrong during tokenizing or parsing
// ============================================================
// Rust has no exceptions. Anything that can fail returns a
// Result<T, E>, and we define E here.
//
// `usize` is Rust's default integer type for indices — it's
// unsigned and sized to match your platform's pointer width.

#[derive(Debug)]
enum JsonError {
    UnexpectedChar(char, usize), // bad character + byte position
    UnexpectedEnd,               // input ended too soon
    InvalidNumber(String),       // e.g. "12.34.56"
    InvalidKeyword(String),      // e.g. "truee"
}

// ============================================================
// tokenize: turns a &str into a flat Vec<Token>
// ============================================================
// Returns Result because input can be malformed.
// On success: Ok(vec of tokens, ending with Eof)
// On failure: Err(JsonError)
fn tokenize(input: &str) -> Result<Vec<Token>, JsonError> {
    let mut tokens: Vec<Token> = Vec::new();

    // char_indices() yields (byte_position, char) pairs.
    // peekable() lets us look at the next item without consuming it.
    let mut chars = input.char_indices().peekable();

    while let Some(&(i, c)) = chars.peek() {
        match c {
            // ---------- Single-character punctuation ----------
            '{' => {
                chars.next();
                tokens.push(Token::LeftBrace);
            }
            '}' => {
                chars.next();
                tokens.push(Token::RightBrace);
            }
            '[' => {
                chars.next();
                tokens.push(Token::LeftBracket);
            }
            ']' => {
                chars.next();
                tokens.push(Token::RightBracket);
            }
            ':' => {
                chars.next();
                tokens.push(Token::Colon);
            }
            ',' => {
                chars.next();
                tokens.push(Token::Comma);
            }

            // ---------- Whitespace: skip it ----------
            ' ' | '\n' | '\t' | '\r' => {
                chars.next();
            }

            // ---------- String literal: "..." ----------
            '"' => {
                chars.next();

                let mut s: String = String::new();

                loop {
                    match chars.next() {
                        // Closing quote → we're done with this string
                        Some((_, '"')) => break,

                        // Escape sequence — the next char tells us what to emit
                        Some((_, '\\')) => {
                            match chars.next() {
                                Some((_, '"')) => s.push('"'),
                                Some((_, '\\')) => s.push('\\'),
                                Some((_, '/')) => s.push('/'),
                                Some((_, 'n')) => s.push('\n'),
                                Some((_, 'r')) => s.push('\r'),
                                Some((_, 't')) => s.push('\t'),
                                Some((_, 'b')) => s.push('\u{0008}'), // backspace
                                Some((_, 'f')) => s.push('\u{000C}'), // form feed
                                Some((j, c)) => return Err(JsonError::UnexpectedChar(c, j)),
                                None => return Err(JsonError::UnexpectedEnd),
                            }
                        }
                        // Any other character → just append it
                        Some((_, c)) => s.push(c),

                        // Ran out of input before finding the closing quote
                        None => return Err(JsonError::UnexpectedEnd),
                    }
                }

                tokens.push(Token::String(s));
            }

            // ---------- Number: 0-9 or '-' ----------
            '0'..='9' | '-' => {
                let mut num_str = String::new();

                // Consume the first character (- or a digit)
                if let Some((_, c)) = chars.next() {
                    num_str.push(c);
                }

                // Consume the rest: digits, at most one '.', optional 'e'/'E' with optional sign
                let mut seen_dot = false;
                let mut seen_exp = false;

                while let Some(&(_, c)) = chars.peek() {
                    match c {
                        '0'..='9' => {
                            num_str.push(c);
                            chars.next();
                        }
                        '.' if !seen_dot && !seen_exp => {
                            seen_dot = true;
                            num_str.push(c);
                            chars.next();
                        }
                        'e' | 'E' if !seen_exp => {
                            seen_exp = true;
                            num_str.push(c);
                            chars.next();

                            // Optional + or - after the exponent
                            if let Some((_, sign)) = chars.next() {
                                if sign == '+' || sign == '-' {
                                    num_str.push(sign);
                                    chars.next();
                                }
                            }
                        }
                        _ => break, // not part of this number
                    }
                }

                match num_str.parse::<f64>() {
                    Ok(num) => tokens.push(Token::Number(num)),
                    Err(_) => return Err(JsonError::InvalidNumber(num_str)),
                }
            }

            // ---------- Keywords: true, false, null ----------
            't' | 'f' | 'n' => {
                // Consume the entire word (letters only)
                let mut word = String::new();

                while let Some(&(_, c)) = chars.peek() {
                    if c.is_ascii_alphabetic() {
                        word.push(c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                match word.as_str() {
                    "true" => tokens.push(Token::Bool(true)),
                    "false" => tokens.push(Token::Bool(false)),
                    "null" => tokens.push(Token::Null),
                    other => return Err(JsonError::InvalidKeyword(other.to_string())),
                }
            }

            // ---------- Anything else is an error ----------
            _ => return Err(JsonError::UnexpectedChar(c, i)),
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

fn main() {
    let json = JsonValue::Object(HashMap::from([
        ("name".to_string(), JsonValue::String("nervos".to_string())),
        ("count".to_string(), JsonValue::Number(42.0)),
        ("active".to_string(), JsonValue::Boolean(true)),
    ]));
    println!("{:#?}", json);

    let sample_token = Token::LeftBrace;
    println!("{:#?}", sample_token);

    let sample_error = JsonError::UnexpectedChar('@', 5);
    println!("{:#?}", sample_error);

    let result = tokenize("{ }");
    println!("{:#?}", result);

    // let result2 = tokenize("{ @");
    // println!("{:#?}", result2);

    // println!("tokenize(\"hello\"): {:#?}", tokenize("\"hello\""));
    // println!(
    //     "tokenize(\"hello\\nworld\"): {:#?}",
    //     tokenize("\"hello\\nworld\"")
    // );
    // println!(
    //     "tokenize(\"quote: \\\"hi\\\"\"): {:#?}",
    //     tokenize("\"quote: \\\"hi\\\"\"")
    // );
    // println!(
    //     "tokenize(\"unterminated\"): {:#?}",
    //     tokenize("\"unterminated")
    // );

    // println!("{:#?}", tokenize("42"));
    // println!("{:#?}", tokenize("-17"));
    // println!("{:#?}", tokenize("3.14"));
    // println!("{:#?}", tokenize("1e10"));
    // println!("{:#?}", tokenize("2.5e-3"));
    // println!("{:#?}", tokenize("[1, 2, 3]"));

    println!("{:#?}", tokenize("true"));
    println!("{:#?}", tokenize("false"));
    println!("{:#?}", tokenize("null"));
    println!("{:#?}", tokenize("[true, false, null]"));
    println!("{:#?}", tokenize("tru"));
    println!("{:#?}", tokenize("taco"));
}

// ============================================================
// TESTS
// ============================================================
// The `#[cfg(test)]` attribute tells the compiler: "only compile
// this module when running `cargo test`, not for regular builds."
//
// The `mod tests` wraps everything in a private module — a
// common Rust convention so tests don't pollute the main scope.
#[cfg(test)]
mod tests {
    use super::*; // brings all public items from the outer module into scope

    // ---------- Punctuation ----------

    #[test]
    fn empty_braces() {
        assert_eq!(
            tokenize("{}"),
            Ok(vec![Token::LeftBrace, Token::RightBrace, Token::Eof])
        )
    }

    #[test]
    fn empty_brackets {
        assert_eq!(
            tokenize("[]"),
            Ok(vec![Token::LeftBracket, Token::RightBracket, Token::Eof])
        )
    }

    #[test]
    fn colon_and_comma() {
        assert_eq!(
            tokenize(":, ,:"),
            Ok(vec![
                Token::Colon, Token::Comma, Token::Comma, Token::Colon, Token::Eof
            ])
        );
    }

    // ---------- Whitespace ----------
    fn whitespace_is_skipped() {
        assert_eq!(
            tokenize("      {  \n\t  }      "),
            Ok(vec![Token::LeftBrace, Token::RightBrace, Token::Eof])
        )
    }

    // ---------- Strings ----------
    #[test]
    fn simple_string() { 
        assert_eq!(
            tokenize("\"hello\""),
            Ok(vec![Token::String("hello".to_string()), Token::Eof])
        )
    }

    #[test]
    fn empty_string() {
        assert_eq!(
            tokenize("\"\""),
            Ok(vec![Token::String(String::new()), Token::Eof])
        );
    }

    #[test]
    fn string_with_escapes() {
        assert_eq!(
            tokenize("\"a\\nb\\tc\""),
            Ok(vec![Token::String("a\nb\tc".to_string()), Token::Eof])
        );
    }

    #[test]
    fn string_with_escaped_quote() {
        assert_eq!(
            tokenize("\"she said \\\"hi\\\"\""),
            Ok(vec![
                Token::String("she said \"hi\"".to_string()),
                Token::Eof
            ])
        );
    }

    #[test]
    fn unterminated_string() {
        assert_eq!(tokenize("\"hello"), Err(JsonError::UnexpectedEnd));
    }

    // ---------- Numbers ----------

    #[test]
    fn integer() {
        assert_eq!(
            tokenize("42"),
            Ok(vec![Token::Number(42.0), Token::Eof])
        );
    }

    #[test]
    fn negative_integer() {
        assert_eq!(
            tokenize("-17"),
            Ok(vec![Token::Number(-17.0), Token::Eof])
        );
    }

    #[test]
    fn decimal() {
        assert_eq!(
            tokenize("3.14"),
            Ok(vec![Token::Number(3.14), Token::Eof])
        );
    }

    #[test]
    fn exponent() {
        assert_eq!(
            tokenize("1e10"),
            Ok(vec![Token::Number(1e10), Token::Eof])
        );
    }

    #[test]
    fn negative_exponent() {
        assert_eq!(
            tokenize("2.5e-3"),
            Ok(vec![Token::Number(2.5e-3), Token::Eof])
        );
    }

    // ---------- Keywords ----------

    #[test]
    fn keyword_true() {
        assert_eq!(
            tokenize("true"),
            Ok(vec![Token::Bool(true), Token::Eof])
        );
    }

    #[test]
    fn keyword_false() {
        assert_eq!(
            tokenize("false"),
            Ok(vec![Token::Bool(false), Token::Eof])
        );
    }

    #[test]
    fn keyword_null() {
        assert_eq!(
            tokenize("null"),
            Ok(vec![Token::Null, Token::Eof])
        );
    }

    #[test]
    fn invalid_keyword() {
        assert_eq!(
            tokenize("taco"),
            Err(JsonError::InvalidKeyword("taco".to_string()))
        );
    }

    // ---------- Errors ----------

    #[test]
    fn unexpected_char() {
        assert_eq!(
            tokenize("@"),
            Err(JsonError::UnexpectedChar('@', 0))
        );
    }

    // ---------- A combined case ----------

    #[test]
    fn array_of_mixed_values() {
        assert_eq!(
            tokenize("[true, 42, \"hi\"]"),
            Ok(vec![
                Token::LeftBracket,
                Token::Bool(true),
                Token::Comma,
                Token::Number(42.0),
                Token::Comma,
                Token::String("hi".to_string()),
                Token::RightBracket,
                Token::Eof,
            ])
        );
    }
}
