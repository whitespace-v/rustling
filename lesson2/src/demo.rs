// statement - instructions without return
fn main() {
    println!("Hello, world!");
    let a = 32 + 3;
    let b = -32;
    let result = demo(a, b);
    demo_one(a, b);
    let mut counter = 0;
    loop {
        // break 42; break and return 42
        // break ; //simple break
        if counter == 10 {
            break 42;
        }
        counter += 1;
    }

    while counter < 100 {
        counter += 1;
    }
    let a = [1, 2, 3, 4, 5, 6];
    for el in a {
        println!("{el}");
    }
}

// expression - evaluating with return
fn demo(a: u8, b: i32) -> i32 {
    println!("Hello, world!");
    // will return without semicolon
    b * 2
}

fn demo_one(a: u8, b: i32) -> bool {
    println!("Hello, world!");
    if a > 10 {
        true
    } else {
        false
    }
}
