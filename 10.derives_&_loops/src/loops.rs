//  LOOPs In Rust

fn main() {
    // LOOP: this runs until a break is applied
    let mut count = 0;
    loop {
        count += 1;
        println!("count: {}", count);
        if count == 5 {
            break;
        }
    }
    // we can also return a value from a loop
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 5 {
            break count;
        }
    };
    println!("result: {}", result);

    //  ----------------------------------------
    //  WHILE LOOP: this runs until a condition is met
    let mut n = 10;
    while n > 0 {
        println!("{}", n);
        n -= 1;
    }

    //  ----------------------------------------
    //  FOR LOOP: this runs until a condition is met
    for i in 0..10 {
        println!("i: {}", i);
    }

    let names = vec!["Alice", "Bob", "Charlie"];
    for name in &names {
        println!("name: {}", name);
    }
    println!("names: {:?}", names);

    //  ----------------------------------------
    // WHILE LET: this is a loop used together with match, it stops when the next value is None
    let mut chars = "abc".chars();

    while let Some(c) = chars.next() {
        println!("{}", c); // prints a, b, c
    }
    // loop ends when chars.next() returns None

    //  ----------------------------------------
    // FOR LOOP with ENUMERATE: If you need both the index and the value
    let friends = vec!["Alice", "Bob", "Charlie"];
    for (i, friend) in friends.iter().enumerate() {
        println!("{} is friend number {}", friend, i + 1);
    }

    let x = Some(5);
    println!("x: {:?}", x.unwrap());
}
