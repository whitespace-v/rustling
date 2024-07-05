use std::num::Wrapping;

fn main() {
    let a: f64 = 0.1;
    let b: f64 = 0.2;
    let c: f64 = 0.3;

    let error_margin32 = f32::EPSILON;
    let error_margin64 = f64::EPSILON;

    if (a + b - c).abs() < error_margin64 {
        println!("y");
        println!("{error_margin64}");
        println!("{error_margin32}");

        // 0.0000000000000002220446049250313
        // 0.00000011920929
    } else {
        println!("n")
    }
}

fn q5() {
    let mut counter = Wrapping(0i8);
    loop {
        println!("{counter}");
        counter += 1;
        // counter = counter.checked_add(1).expect("fail!"); // if number without wrapping and dont want to panic by overflow
    }
}
// предел точности, двойная точность в f64 и погрешность в эпсилон и флоат типах

fn q6() {
    let s = "П Р И В Е Т !";

    println!("{}", s.len()); //bytes
    println!("{}", s.chars().count()) // letters
}
