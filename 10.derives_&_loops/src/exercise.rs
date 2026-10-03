
#[derive(Debug, PartialEq)]
struct Temprature {
    degrees: f64,
    unit: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Task {
    title: String,
    done: bool,
}

fn main() {
    // let temp1 = Temprature {
    //     degrees: 22.0,
    //     unit: String::from("C"),
    // };
    // let temp2 = Temprature {
    //     degrees: 70.0,
    //     unit: String::from("F"),
    // };
//     println!("Temperature: {:?}", temp1);
//     println!("Temperature: {:?}", temp2);

//     println!("Temperature: {:?}", temp1 == temp2);

    let task = Task {
        title: String::from("Buy groceries"),
        done: false,
    };
    let task2 = task;
    println!("Task: {:?}", task);
    println!("Task2: {:?}", task2);

}