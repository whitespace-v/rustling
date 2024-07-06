use std::result;

#[derive(Debug)]
enum OrderStatus {
    Paid { amount: u32 }, // named field
    Sent,
    Delivered,
    Disputed(String), // nameless field
}

//impl for enum
impl OrderStatus {
    fn info(&self) {
        // must covered all items enumerated
        match self {
            // named field
            Self::Paid { amount } => println!("Paid, {amount}"),
            Self::Sent => println!("Sent !"),
            Self::Delivered => println!("Delivered !"),
            // nameless field
            Self::Disputed(reason) => println!("Disput: {reason}"),
        }
    }
}

//combine enums and struct
#[derive(Debug)]
struct Order {
    customer: String,
    status: OrderStatus,
}

fn main() {
    let status = OrderStatus::Sent;
    // demo(&status);
    let order = Order {
        customer: String::from("Order N1"),
        status,
    };
    let status2 = OrderStatus::Disputed(String::from("broken"));
    let paid = OrderStatus::Paid { amount: 32 };
    // println!("{status2:#?} {order:#?} {paid:#?}");
    status2.info();
    paid.info();

    // Option built-in enum, so we can operate it like with its
    let value: Option<i8> = Some(32);
    println!("{value:?}");
    // let result = value + 5; // value its option, 5 -> int, we cant use it
    // so we can unwrap it, with default value if None
    let result = value.unwrap_or(0) + 5;
    println!("{result}");
    let value: Option<i8> = None;
    println!("{value:?}");

    // btw we can use match
    match value {
        // if value in Option "value"
        Some(a) => {
            let result = a + 5;
            println!("Result: {result}")
        }
        // if None in Option "value"
        _ => (),
    }

    // we can use it more comfy:
    // pseudo: if a is a value, so...
    if let Some(a) = value {
        let result = a + 5;
        println!("Result: {result}")
    }
}

fn demo(status: &OrderStatus) {
    println!("{status:#?}");
}
