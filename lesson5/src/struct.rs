// trait, gave our struct possibility to out in terminal
#[derive(Debug)]
struct Car {
    brand: String,
    max_speed: u16,
    max_gas: f32,
    current_gas: f32,
    gas_consumption: f32,
}

impl Car {
    fn new(
        brand: &str,
        max_speed: u16,
        max_gas: f32,
        current_gas: f32,
        gas_consumption: f32,
    ) -> Self {
        Self {
            brand: String::from(brand),
            max_speed,
            max_gas,
            current_gas,
            gas_consumption,
        }
    }
    fn drive(&mut self, distance: f32) {
        let total_gas_consumed = distance * self.gas_per_km();

        if total_gas_consumed > self.current_gas {
            println!("not enough gas !");
        } else {
            println!("Driving");
            self.current_gas -= total_gas_consumed
        }
    }

    fn gas_per_km(&self) -> f32 {
        self.gas_consumption / 100.0
    }
    fn is_faster(&self, car: &Car) -> bool {
        self.max_speed > car.max_speed
    }
}

// classic C struct -> include named fields
struct Point {
    x: f32,
    y: f32,
}
// Tuple struct -> include nameless fields
struct Color(i32, i32, i32);
// Unit Struct -> fieldless
struct Always;

fn main() {
    let mut car = Car {
        brand: String::from("Ford"),
        max_speed: 120,
        max_gas: 55.0,
        current_gas: 55.0,
        gas_consumption: 23.0,
    };

    let car1 = Car::new("Volvo", 150, 75.0, 10.0, 25.0);
    let car2 = Car {
        current_gas: 0.0,
        ..car1 // RangeFull operator (clice all)
    };
    println!("{}", car.brand);
    println!("{car:#?}");
    let distance = 40.0;

    // drive(&mut car, distance as f32);
    car.drive(distance);
    println!("{}", car.is_faster(&car2))
}

// fn drive(car: &mut Car, distance: f32) {
//     let gas_per_km = car.gas_consumption / 100.0;

//     let total_gas_consumed = distance * gas_per_km;

//     if total_gas_consumed > car.current_gas {
//         println!("not enough gas !");
//     } else {
//         println!("Driving");
//         car.current_gas -= total_gas_consumed
//     }
// }
