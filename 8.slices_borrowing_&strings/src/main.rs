// using &String - it is restrictive  only to &String
fn my_firstname (s: &String){
    println!("My Firstname is {}", s);
}

// Using &str - it is more flexible and can be used with any type that implements the Deref trait
fn my_lastname (s: &String){
    println!("My Lastname is {}", s);
}

// testing Lifetime
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() > b.len() { a } else { b }
}

fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(i) => &s[0..i],   // slice from start up to the space
        None => s,             // no space — whole string is one word
    }
}

fn main() {
    let s = String::from("Hello world!");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{} {}", hello, world);

    println!("First word: {}", first_word(&s));

    let first_name = String::from("John");
    let last_name = String::from("Doe");
    my_firstname(&first_name);
   // my_firstname(&first_name as &str); // will not work 
    my_lastname(&last_name as &String); // will work because &String implements the Deref trait

    let result = longest(hello, world);
    println!("The longest string is {}", result);
}


// fn main() {
//     let slice;
//     {
//         let sentence = String::from("hello rust world");
//         slice = &sentence[0..5];
//     }   // sentence is dropped HERE
//     println!("{}", slice);   // ERROR: `sentence` does not live long enough
// }