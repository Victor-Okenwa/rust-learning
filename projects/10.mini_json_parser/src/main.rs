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
    Boolean(bool),
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
                todo!("keyword tokenizing — Part 2d")
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

    println!("tokenize(\"hello\"): {:#?}", tokenize("\"hello\""));
    println!(
        "tokenize(\"hello\\nworld\"): {:#?}",
        tokenize("\"hello\\nworld\"")
    );
    println!(
        "tokenize(\"quote: \\\"hi\\\"\"): {:#?}",
        tokenize("\"quote: \\\"hi\\\"\"")
    );
    println!(
        "tokenize(\"unterminated\"): {:#?}",
        tokenize("\"unterminated")
    );

    println!("{:#?}", tokenize("42"));
    println!("{:#?}", tokenize("-17"));
    println!("{:#?}", tokenize("3.14"));
    println!("{:#?}", tokenize("1e10"));
    println!("{:#?}", tokenize("2.5e-3"));
    println!("{:#?}", tokenize("[1, 2, 3]"));
}
