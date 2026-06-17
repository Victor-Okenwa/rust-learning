fn main() {
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte =  b'A';
    println! ("decimal: {}", decimal);
    println!("hex: {}", hex);
    println! ("octal: {}", octal);
    println! ("binary: {}", binary);
    println! ("byte: {}", byte);


    for char in "DANIel PeterSeN".chars(){
    println!("{}", char)
    }

    let tup: (i32, f64, char) = (500, 6.753, 'y');

    println!("{:?}", tup);
}
