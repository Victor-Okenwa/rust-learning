fn first_word(s: &String) -> &str {
    return &s[..5];
}

fn main() {
    let mut s = String::from("hello");
    let r1 = &mut s;
    r1.push_str(" world");
    let r2 = &s;
    println!("{}", r2);

    let new_string = String::from("hello world");
    let word = first_word(&new_string);
    println!("{}", word);
}