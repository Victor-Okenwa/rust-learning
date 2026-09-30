fn dangle() -> &String {
    let s = String::from("oops");
    &s   // ERROR: s is dropped, but we're returning a reference to it
}

fn main() {
    // BORROWING - immutable
    let a = String::from("nervos");
    let b = &a;
    let c = &a;
    println!("a: {}, b: {}, c: {}", a, b, c);

    // BORROWING - mutable
    let mut d = String::from("fuydhvkvfd");
    let e = &mut d;
    e.push_str(" is cool");
    println!("d: {}", d);


}
