use std::io; //input out
const C: f32 = 32.0;
fn c_to_f(celsius_temp: f32) -> f32 {
    (celsius_temp * (9.0 / 5.0)) + C
}

fn f_to_c(celsius_temp: f32) -> f32 {
    (celsius_temp * (5.0 / 9.0)) + C
}

// Option - может вернуть а может и не вернуть
fn convert(temperature: f32, choice: u8) -> Option<f32> {
    match choice {
        1 => Some(c_to_f(temperature)),
        2 => Some(f_to_c(temperature)),
        _ => None,
    }
}

fn io_choice() -> u8 {
    println!("Temperature converter, \n (1) C to F \n (2) F to C");
    let mut user_choice = String::new();
    io::stdin().read_line(&mut user_choice).unwrap();
    let n_choice = user_choice
        .trim()
        .parse::<u8>()
        .expect("Please type a number");
    return n_choice;
}

fn io_temperature() -> f32 {
    println!("Enter temperature:");
    let mut temperature = String::new();
    io::stdin().read_line(&mut temperature).unwrap();
    let temperature = temperature
        .trim()
        .parse::<f32>()
        .expect("Pls type a number !");
    return temperature;
}

pub fn main() {
    let n_choice = io_choice();
    let temperature = io_temperature();
    match convert(temperature, n_choice) {
        Some(result) => println!("The  result: {result}"),
        None => println!("Unknown precision"),
    }
}
