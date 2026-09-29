fn celcius_to_fahrenheit(celcius: f64) -> f64 {
    (celcius * 9.0/5.0) + 32.0
}

fn fahrenheit_to_celcius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0/9.0
}

fn main() {
    let celcius = 25.0;
    let fahrenheit = 77.0;

    let fahrenheit_result = celcius_to_fahrenheit(celcius);
    let celcius_result = fahrenheit_to_celcius(fahrenheit);

    println!("{}°C is {}°F", celcius, fahrenheit_result);
    println!("{}°F is {}°C", fahrenheit, celcius_result);
}
