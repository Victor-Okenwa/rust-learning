


fn main() {
    let mut counter = 0;
    let mut snapshot = counter;
    counter += 1;
    println!("Snapshot: {}, Counter: {}", snapshot, counter);
}
