use std::collections::HashMap;

// ============================================================
// JsonValue: the "target" type — what a parsed JSON becomes
// ============================================================
// JSON has exactly six kinds of values. Each variant of this
// enum represents one kind, and carries the data that kind holds.

#[derive(Debug, PartialEq)]
enum JsonValue {
    Null,
    Bool(bool),
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq)]
enum JsonError {
    UnexpectedChar(char, usize), // bad character + byte position
    UnexpectedEnd,               // input ended too soon
    InvalidNumber(String),       // e.g. "12.34.56"
    InvalidKeyword(String),      // e.g. "truee"
    UnexpectedToken { expected: Token, found: Token }, // e.g. expected LeftBrace, got RightBrace
    ExpectedValue(Token),        // expected a value, found this token
    TrailingTokens(Token),       // input continued after the value ended
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
                            if let Some(&(_, sign)) = chars.peek() {
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

// ============================================================
// Parser: walks through tokens and builds a JsonValue
// ============================================================
// It OWNS the tokens (moved in from the tokenizer). The `pos`
// field tracks where we are in the list.

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    // Constructor — like `new` in other languages
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    // Look at the current token without advancing
    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }

    // Return the current token AND move forward
    fn advance(&mut self) -> Token {
        let tok = self.tokens[self.pos].clone();
        self.pos += 1;
        tok
    }

    // Consume the current token if it matches, else error
    fn expect(&mut self, expected: &Token) -> Result<(), JsonError> {
        if self.peek() == expected {
            self.pos += 1;
            Ok(())
        } else {
            Err(JsonError::UnexpectedToken {
                expected: self.peek().clone(),
                found: self.peek().clone(),
            })
        }
    }

    // Are we at the end (Eof token)?
    fn at_end(&self) -> bool {
        matches!(self.peek(), Token::Eof)
    }

    // Parse a single JSON value (Null, Boolean, Number, String, Array, Object)
    fn parse_value(&mut self) -> Result<JsonValue, JsonError> {
        match self.peek().clone() {
            Token::Null => {
                self.advance();
                Ok(JsonValue::Null)
            }
            Token::Bool(b) => {
                self.advance();
                Ok(JsonValue::Bool(b))
            }
            Token::Number(n) => {
                self.advance();
                Ok(JsonValue::Number(n))
            }
            Token::String(s) => {
                self.advance();
                Ok(JsonValue::String(s))
            }
            Token::LeftBracket => self.parse_array(),
            Token::LeftBrace => self.parse_object(),
            other => Err(JsonError::ExpectedValue(other)),
        }
    }

    // Parse an array: [value, value, ...]
    fn parse_array(&mut self) -> Result<JsonValue, JsonError> {
        self.advance(); // consume '['

        let mut items = Vec::new();
        // Empty Array
        if matches!(self.peek(), Token::RightBracket) {
            self.advance();
            return Ok(JsonValue::Array(items));
        }

        loop {
            // Parse one value (recursion!)
            let value = self.parse_value()?;
            items.push(value);

            // After a value, the next token must be ',' or ']'
            match self.peek() {
                Token::Comma => {
                    self.advance();
                }

                Token::RightBracket => {
                    self.advance();
                    break;
                }

                other => {
                    return Err(JsonError::UnexpectedToken {
                        expected: Token::Comma, // or ']'
                        found: other.clone(),
                    });
                }
            }
        }

        Ok(JsonValue::Array(items))
    }

    fn parse_object(&mut self) -> Result<JsonValue, JsonError> {
        self.advance(); // consume '{'

        let mut map = HashMap::new();

        // Empty Object
        if matches!(self.peek(), Token::RightBrace) {
            self.advance();
            return Ok(JsonValue::Object(map));
        }

        loop {
            // --- Parse the key ---
            let key = match self.peek().clone() {
                Token::String(s) => {
                    self.advance();
                    s
                }
                other => {
                    return Err(JsonError::UnexpectedToken {
                        expected: Token::String(String::new()),
                        found: other,
                    });
                }
            };

            // --- Expect a colon ---
            match self.peek() {
                Token::Colon => {
                    self.advance();
                }
                other => {
                    return Err(JsonError::UnexpectedToken {
                        expected: Token::Colon,
                        found: other.clone(),
                    });
                }
            }

            // --- Parse the value (recursion again!) ---
            let value = self.parse_value()?;
            map.insert(key, value);

            // --- After a value, the next token must be ',' or '}' ---

            match self.peek() {
                Token::Comma => {
                    self.advance();
                }
                Token::RightBrace => {
                    self.advance();
                    break;
                }
                other => {
                    return Err(JsonError::UnexpectedToken {
                        expected: Token::Comma,
                        found: other.clone(),
                    });
                }
            }
        }
        Ok(JsonValue::Object(map))
    }
}

fn parse(tokens: Vec<Token>) -> Result<JsonValue, JsonError> {
    let mut parser = Parser::new(tokens);
    let value = parser.parse_value()?;
    if !parser.at_end() {
        Err(JsonError::TrailingTokens(parser.peek().clone()))
    } else {
        Ok(value)
    }
}

fn main() {
    let json = JsonValue::Object(HashMap::from([
        ("name".to_string(), JsonValue::String("nervos".to_string())),
        ("count".to_string(), JsonValue::Number(42.0)),
        ("active".to_string(), JsonValue::Bool(true)),
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

    let tokens = tokenize("{ }").unwrap();
    let mut parser = Parser::new(tokens);

    println!("peek: {:?}", parser.peek()); // LeftBrace
    println!("advance: {:?}", parser.advance()); // LeftBrace
    println!("peek: {:?}", parser.peek()); // RightBrace
    println!("at_end: {}", parser.at_end()); // false

    let result = parser.expect(&Token::RightBrace);
    println!("expect: {:?}", result); // Ok(())
    println!("at_end: {}", parser.at_end()); // true

    println!("{:#?}", parse(tokenize("42").unwrap()));
    println!("{:#?}", parse(tokenize("true").unwrap()));
    println!("{:#?}", parse(tokenize("null").unwrap()));
    println!("{:#?}", parse(tokenize("\"hello\"").unwrap()));

    // Error cases
    println!("{:#?}", parse(tokenize("").unwrap())); // empty — Eof is not a value
    println!("{:#?}", parse(tokenize("42 43").unwrap())); // trailing

    println!("{:#?}", parse(tokenize("[]").unwrap()));
    println!("{:#?}", parse(tokenize("[1, 2, 3]").unwrap()));
    println!("{:#?}", parse(tokenize("[true, null, \"hi\"]").unwrap()));
    println!("{:#?}", parse(tokenize("[[1, 2], [3, 4]]").unwrap())); // nested!

    // Errors
    println!("{:#?}", parse(tokenize("[1,]").unwrap())); // trailing comma
    println!("{:#?}", parse(tokenize("[1 2]").unwrap())); // missing comma

    println!("{:#?}", parse(tokenize("{}").unwrap()));
    println!("{:#?}", parse(tokenize("{\"a\": 1}").unwrap()));
    println!("{:#?}", parse(tokenize("{\"a\": 1, \"b\": 2}").unwrap()));
    println!(
        "{:#?}",
        parse(tokenize("{\"nested\": {\"x\": [1, 2]}}").unwrap())
    );

    // Errors
    println!("{:#?}", parse(tokenize("{1: 2}").unwrap())); // non-string key
    println!("{:#?}", parse(tokenize("{\"a\" 1}").unwrap())); // missing colon
    println!("{:#?}", parse(tokenize("{\"a\": 1 \"b\": 2}").unwrap())); // missing comma
    println!("{:#?}", parse(tokenize("{\"a\": }").unwrap()));
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
    fn empty_brackets() {
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
                Token::Colon,
                Token::Comma,
                Token::Comma,
                Token::Colon,
                Token::Eof
            ])
        );
    }

    // ---------- Whitespace ----------
    #[test]
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
        assert_eq!(tokenize("42"), Ok(vec![Token::Number(42.0), Token::Eof]));
    }

    #[test]
    fn negative_integer() {
        assert_eq!(tokenize("-17"), Ok(vec![Token::Number(-17.0), Token::Eof]));
    }

    #[test]
    fn decimal() {
        assert_eq!(tokenize("3.14"), Ok(vec![Token::Number(3.14), Token::Eof]));
    }

    #[test]
    fn exponent() {
        assert_eq!(tokenize("1e10"), Ok(vec![Token::Number(1e10), Token::Eof]));
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
        assert_eq!(tokenize("true"), Ok(vec![Token::Bool(true), Token::Eof]));
    }

    #[test]
    fn keyword_false() {
        assert_eq!(tokenize("false"), Ok(vec![Token::Bool(false), Token::Eof]));
    }

    #[test]
    fn keyword_null() {
        assert_eq!(tokenize("null"), Ok(vec![Token::Null, Token::Eof]));
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
        assert_eq!(tokenize("@"), Err(JsonError::UnexpectedChar('@', 0)));
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

    // ============================================================
    // PARSER TESTS
    // ============================================================
    // Helper: tokenize + parse in one step, panicking on tokenizer error
    fn p(input: &str) -> Result<JsonValue, JsonError> {
        parse(tokenize(input).unwrap())
    }

    // ---------- Simple values ----------

    #[test]
    fn parse_null() {
        assert_eq!(p("null"), Ok(JsonValue::Null));
    }

    #[test]
    fn parse_true() {
        assert_eq!(p("true"), Ok(JsonValue::Bool(true)));
    }

    #[test]
    fn parse_false() {
        assert_eq!(p("false"), Ok(JsonValue::Bool(false)));
    }

    #[test]
    fn parse_number() {
        assert_eq!(p("42"), Ok(JsonValue::Number(42.0)));
    }

    #[test]
    fn parse_string() {
        assert_eq!(p("\"hello\""), Ok(JsonValue::String("hello".to_string())));
    }

    // ---------- Empty collections ----------

    #[test]
    fn parse_empty_array() {
        assert_eq!(p("[]"), Ok(JsonValue::Array(vec![])));
    }

    #[test]
    fn parse_empty_object() {
        assert_eq!(p("{}"), Ok(JsonValue::Object(HashMap::new())));
    }

    // ---------- Arrays ----------

    #[test]
    fn parse_simple_array() {
        assert_eq!(
            p("[1, 2, 3]"),
            Ok(JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0),
            ]))
        );
    }

    #[test]
    fn parse_mixed_array() {
        assert_eq!(
            p("[true, null, \"hi\"]"),
            Ok(JsonValue::Array(vec![
                JsonValue::Bool(true),
                JsonValue::Null,
                JsonValue::String("hi".to_string()),
            ]))
        );
    }

    #[test]
    fn parse_nested_array() {
        assert_eq!(
            p("[[1, 2], [3, 4]]"),
            Ok(JsonValue::Array(vec![
                JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0),]),
                JsonValue::Array(vec![JsonValue::Number(3.0), JsonValue::Number(4.0),]),
            ]))
        );
    }

    // ---------- Objects ----------

    #[test]
    fn parse_simple_object() {
        let mut expected = HashMap::new();
        expected.insert("a".to_string(), JsonValue::Number(1.0));

        assert_eq!(p("{\"a\": 1}"), Ok(JsonValue::Object(expected)));
    }

    #[test]
    fn parse_object_with_multiple_keys() {
        let mut expected = HashMap::new();
        expected.insert("a".to_string(), JsonValue::Number(1.0));
        expected.insert("b".to_string(), JsonValue::Bool(true));

        assert_eq!(
            p("{\"a\": 1, \"b\": true}"),
            Ok(JsonValue::Object(expected))
        );
    }

    #[test]
    fn parse_nested_object() {
        let mut inner = HashMap::new();
        inner.insert(
            "x".to_string(),
            JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0)]),
        );

        let mut outer = HashMap::new();
        outer.insert("nested".to_string(), JsonValue::Object(inner));

        assert_eq!(
            p("{\"nested\": {\"x\": [1, 2]}}"),
            Ok(JsonValue::Object(outer))
        );
    }

    // ---------- Error cases ----------

    #[test]
    fn parse_empty_input_errors() {
        assert_eq!(p(""), Err(JsonError::ExpectedValue(Token::Eof)));
    }

    #[test]
    fn parse_trailing_tokens_errors() {
        assert_eq!(
            p("42 43"),
            Err(JsonError::TrailingTokens(Token::Number(43.0)))
        );
    }

    #[test]
    fn parse_unclosed_array_errors() {
        assert!(matches!(p("[1"), Err(JsonError::UnexpectedToken { .. })));
    }

    #[test]
    fn parse_unclosed_object_errors() {
        assert!(matches!(
            p("{\"a\": 1"),
            Err(JsonError::UnexpectedToken { .. })
        ));
    }

    #[test]
    fn parse_non_string_key_errors() {
        assert!(matches!(
            p("{1: 2}"),
            Err(JsonError::UnexpectedToken { .. })
        ));
    }

    #[test]
    fn parse_missing_colon_errors() {
        assert!(matches!(
            p("{\"a\" 1}"),
            Err(JsonError::UnexpectedToken { .. })
        ));
    }
}
