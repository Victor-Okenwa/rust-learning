// ============================================================
// DERIVES: the "usual extras" macro for your types
// ============================================================
// `#[derive(...)]` auto-generates boilerplate code for a type.
// Each derive adds one capability. You can list several at once.
// The compiler will refuse to derive something that doesn't make sense.

use std::collections::HashMap;

// ============================================================
// 1. Debug
// ============================================================
// Generates `{:?}` and `{:#?}` formatting.
// You use it constantly to inspect values.
// It does NOT change how the type works at runtime — it only
// adds a way to PRINT the value for debugging.

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Debug can also be derived on enums:
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8), // carries three bytes (R, G, B)
}

// ============================================================
// 2. Clone
// ============================================================
// Adds a `.clone()` method that makes a DEEP copy.
// "Deep" means heap data (String, Vec, HashMap) is also duplicated.
// You must call `.clone()` explicitly — the compiler won't do it for you.

#[derive(Debug, Clone)]
struct Message {
    text: String,  // String owns heap data, so cloning copies it
    priority: i32, // simple, cheap to copy
}

// ============================================================
// 3. Copy
// ============================================================
// Makes the type behave like `i32`: assignment and passing to
// functions COPY the value instead of MOVING it.
// RULE: Copy can ONLY be derived if every field is also Copy.
// String, Vec, HashMap are NOT Copy — they own heap data.

#[derive(Debug, Clone, Copy)]
struct Coordinates {
    lat: f64,
    lng: f64,
}

// This next line WOULD NOT COMPILE, because String is not Copy:
//
// #[derive(Debug, Clone, Copy)]
// struct Bad {
//     name: String,
// }

// ============================================================
// 4. PartialEq and Eq
// ============================================================
// PartialEq gives you `==` and `!=` between two values.
// Eq is a marker that says "equality is total" (like for numbers).
// Most of the time you derive BOTH together.
// Note: f64 is only PartialEq, not Eq, because NaN != NaN.
//       So any struct/enum containing f64 can derive PartialEq but not Eq.

#[derive(Debug, PartialEq)]
struct Version {
    major: u32,
    minor: u32,
}

// ============================================================
// 5. PartialOrd and Ord
// ============================================================
// PartialOrd gives you `<`, `>`, `<=`, `>=`.
// Ord is a marker saying "ordering is total" (no NaN weirdness).
// Requires PartialEq (and often Eq) too.

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Score {
    points: u32,
}

// ============================================================
// 6. Hash
// ============================================================
// Makes the type usable as a key in HashMap or HashSet.
// Requires Eq as well (because HashMap relies on stable equality).

#[derive(Debug, PartialEq, Eq, Hash)]
struct UserId {
    id: u64,
}

// ============================================================
// 7. Default
// ============================================================
// Adds a `Default::default()` associated function that returns
// the "zero-like" value of the type.
// Integers default to 0, booleans to false, String to "", Vec to [].

#[derive(Debug, Default)]
struct Settings {
    volume: u8,
    muted: bool,
    name: String,
}

// ============================================================
// MAIN: exercise each derive
// ============================================================

fn main() {
    // ---------- Debug ----------
    let point = Point { x: 1, y: 2 };
    println!("Point: {:?}", point);
    println!("Point: {:#?}", point);

    let c = Color::Custom(255, 128, 0);
    println!("Color: {:?}", c);

    // ---------- Clone ----------
    let original = Message {
        text: String::from("hello"),
        priority: 1,
    };
    let copy = original.clone(); // deep copy
    println!("original: {:?}", original); // original is still alive
    println!("copy:     {:?}", copy);

    // ---------- Copy ----------
    let a = Coordinates { lat: 6.5, lng: 3.4 };
    let b = a; // Copy happens implicitly — no .clone() needed
    println!("a = {:?}, b = {:?}", a, b); // both alive, both usable

    // ---------- PartialEq ----------
    let v1 = Version { major: 1, minor: 0 };
    let v2 = Version { major: 1, minor: 0 };
    let v3 = Version { major: 2, minor: 0 };
    println!("v1 == v2: {}", v1 == v2); // true
    println!("v1 == v3: {}", v1 == v3); // false

    // ---------- PartialOrd / Ord ----------
    let s1 = Score { points: 10 };
    let s2 = Score { points: 20 };
    println!("s1 < s2: {}", s1 < s2); // true
    println!("s2 > s1: {}", s2 > s1); // true

    // You can even sort them:
    let mut scores = vec![
        Score { points: 30 },
        Score { points: 10 },
        Score { points: 20 },
    ];
    scores.sort(); // uses Ord
    println!("sorted: {:?}", scores);

    // ---------- Hash ----------
    let mut votes: HashMap<UserId, u32> = HashMap::new();
    votes.insert(UserId { id: 1 }, 100);
    votes.insert(UserId { id: 2 }, 250);
    println!("votes: {:?}", votes);

    // ---------- Default ----------
    let default_settings = Settings::default();
    println!("defaults: {:?}", default_settings);
    // volume = 0, muted = false, name = "" — all "zero-like"

    // You can also override individual fields:
    let custom_settings = Settings {
        volume: 50,
        ..Default::default() // "fill the rest from default"
    };
    println!("custom:   {:?}", custom_settings);
}
