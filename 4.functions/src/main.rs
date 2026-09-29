fn another_function(a: String){
    println!("{}", a);
}

fn sum (num_1: i32, num_2: i32) -> i32 {
     return num_1 + num_2;
}

fn sum_diff (num_1: i32, num_2: i32) -> (i32, i32) {
(num_1+ num_2, num_1 - num_2)
}

fn main() {
    println!("Hello, world!");
    another_function("I am another function".to_string());


    // STATEMENTS
    let x = 50;
    let y = {
        let x= 50;
        x + 10
    };

    let z = x+y; // Expression

    println!("{}", z);

    let sum_result = sum(5, 10);
    println!("{}", sum_result);

    let sum_diff_result = sum_diff(5, 10);
    println!("sum = {}, diff = {}", sum_diff_result.0, sum_diff_result.1);
}