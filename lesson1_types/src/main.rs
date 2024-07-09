mod types;
fn main() {
    let a = 90000;
    println!("Hello, world!");

    //SCALAR
    // signed integer
    let b: i128;
    let d: isize; //arch
                  // signed integer
    let s: u8; // unsigned integer - не указываем знак, положительное или отрицательное, >=0
               // можно также заводить как hex, bi, o...
               // let hex = 0xab;
               // let o = 0o;
               // let c = 0b

    // Float
    //f32 тип с одинарной точностью
    //f64 тип с двойной точностью

    // Boolean
    let flag = true;

    // Char
    let c: char = 'a';

    // Tuple
    let my_tuple: (i8, char, bool) = (42, 'f', false);
    let tuple_int = my_tuple.0;

    //  Array
    let arr = [1, 2, 3]; // fixed !
    arr[1];
    types::main()
}
