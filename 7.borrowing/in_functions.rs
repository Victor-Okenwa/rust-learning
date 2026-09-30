// Takes ownership — String is moved in, dropped at end of function
fn consume(s: String) {
    println!("{}", s);
}

// Borrows — String is not moved, caller keeps ownership
fn inspect(s: &String) {
    println!("{}", s);
}

// Borrows mutably — can modify, but caller keeps ownership
fn modify(s: &mut String) {
    s.push_str(" is cool");
}

fn main() {
    let mut greeting = String::from("hello");

    inspect(&greeting);       // borrow
    println!("{}", greeting); // still usable

    modify(&mut greeting);    // mutable borrow
    println!("{}", greeting); // "hello!"

    consume(greeting);        // moved — greeting is done
    // println!("{}", greeting);  // ERROR: moved
}