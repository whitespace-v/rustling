fn main() {
    println!("Hello, world!");
    let vec1 = vec![1, 2, 3, 4, 5, 6, 110];
    let vec2 = vec![2, 5, 6, 12, 2];
    let greater_vec = compare_sums(&vec1, &vec2);
    println!("{greater_vec:?}")
}

// fn sum<T>(numbers: &[T]) -> T
// where
//     T: std::marker::Copy + std::ops::Add<Output = T>,
// {
//     numbers.iter().copied().reduce(|acc, n| acc + n).unwrap()
// }

// lifetime elision rules
// 1. если мы завязываемся на две ссылки - компилятор присвоит им по дефолту разные лайфтаймы
// 2. если мы завязываемся на одной ссылке - компилятор присваивает один лайфтайм на входе и выходе
// 3. Когда на вход прилетает self и какая-то еще ссылка - то он воспринимает лайфтайм self и для всех
//    остальных переменных на входе и выходе

// &'static - референс который будет доступен во всей жизни программы:
//      по дефолту применяется к &str
fn sum<'a, T>(numbers: &'a [T]) -> T
where
    T: std::iter::Sum<&'a T>,
{
    numbers.iter().sum()
}

// link lifetimes of these vectors (output value is valid till all input values are valid)
fn compare_sums<'a>(vec1: &'a Vec<i32>, vec2: &'a Vec<i32>) -> &'a Vec<i32> {
    if sum(vec1) >= sum(vec2) {
        vec1
    } else {
        vec2
    }
}
