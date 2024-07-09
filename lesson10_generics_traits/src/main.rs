// Структура - структура типов данных
// Имплименты - функции в типах данных
// трейты - структура функции

#[derive(Debug)]
struct Employee<T, U> {
    age: T,
    salary: U,
    tax: U,
}
// трейт для копирования, трейт для вычитания в котором будет тип U, трейт для умножения в результате которого будет тип U
// тип Т нигде не фигурирует, поэтому его можно не типизировать
impl<T, U: std::marker::Copy + std::ops::Sub<Output = U> + std::ops::Mul<Output = U>>
    Employee<T, U>
{
    fn salary_with_tax(&self) -> U {
        self.salary - (self.salary * self.tax)
    }
}

trait Drive {
    fn can_drive(&self) -> bool;
}
struct Car {
    gas: u32,
}
impl Drive for Car {
    fn can_drive(&self) -> bool {
        self.gas > 0
    }
}
struct ElectroCar {
    battery_charge: u32,
}

impl Drive for ElectroCar {
    fn can_drive(&self) -> bool {
        self.battery_charge > 0
    }
}

fn car_info<T: Drive>(car: &T) {
    println!("Car drive? {}", car.can_drive());
}

fn car_info_cmp<T: Drive, U: Drive>(car: &T, other_car: &U) {
    println!("Car drive? {}, {}", car.can_drive(), other_car.can_drive());
}

fn main() {
    let numbers: Vec<i32> = vec![32, 11, 2, 43, 3];

    let empl = Employee {
        age: 21,
        salary: 520.0,
        tax: 0.3,
    };
    let salary = empl.salary_with_tax();
    println!("{salary}");
    println!("{empl:?}");
    let car = Car { gas: 5 };
    let car1 = ElectroCar {
        battery_charge: 100,
    };
    car_info(&car);
    car_info_cmp(&car, &car1);

    println!("{}", sum(&numbers));
}

// fn sum<T: std::marker::Copy + std::ops::Add<Output = T>>(numbers: &[T]) -> T {
//     numbers.iter().copied().reduce(|acc, n| acc + n).unwrap()
// }
// equals:
fn sum<T>(numbers: &[T]) -> T
where
    T: std::marker::Copy + std::ops::Add<Output = T>,
{
    numbers.iter().copied().reduce(|acc, n| acc + n).unwrap()
}
