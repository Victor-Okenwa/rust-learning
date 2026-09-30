use std::collections::HashMap;
enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

fn main() {
    // Manually build a tiny JSON value in Rust:
    // { "name": "nervos", "count": 42, "active": true }

    let json = JsonValue::Object(HashMap::from([
        ("name".to_string(), JsonValue::String("nervos".to_string())),
        ("count".to_string(), JsonValue::Number(42.0)),
        ("active".to_string(), JsonValue::Boolean(true)),
    ]));

    println!("{:?}", json);
    // Print it with {:?}
}
