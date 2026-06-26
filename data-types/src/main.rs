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

    println!("{:?}", tup.1);

    let arr = [1, 2, 3, 4, 5];

    println!("{:#?}", arr[1]);


    // STRUCTS

    struct Person {
        name: String,
        age: u8
    }

    let person = Person {
        name: "JOHN".to_string(),
        age: 28
    };

    println!("{:#?}", person.name);

    // ENUMS
    enum TrafficLight {
        Red,
        Yellow,
        Green,
    }

    let light = TrafficLight::Green;

    match light {
        TrafficLight::Red => println!("Stop!"),
        TrafficLight::Yellow => println!("Get ready!"),
        TrafficLight::Green => println!("Go!"),
    }
}
