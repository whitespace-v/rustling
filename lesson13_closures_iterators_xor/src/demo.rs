fn main() {
    println!("Hello, world!");
    let param = 10;
    let value: Option<u8> = None;

    // let res: u8 = value.unwrap_or_else(|| -> u8 { fallback(param) });
    let closure = || -> u8 { fallback(param) };
    let res: u8 = value.unwrap_or_else(closure);
    closure();

    let mut b: u8 = 0;

    let mut demo = |a: u8| {
        println!("{a}");
        b = b + a;
    };
    demo(3);

    let mut v = vec![1, 2, 3];
    let mut process = || v.push(3);

    process();
    &v;

    // ITERATORS:
    let v1: Vec<i32> = vec![1, 2, 3];
    // let mut i = v1.iter();

    // -> Option
    // i.next(); // 1
    // i.next(); // 2
    // i.next(); // 3
    // i.next(); // None

    // let v2: Vec<i32> = i.map(|x| x + 1).collect();
    // let v2: Vec<i32> = i.map(|x| x + 1).take(2).collect(); // iterate only first 2 items and collect it;

    let res = v
        .iter()
        .enumerate() // pairs of index: value
        .filter(|(i, _)| *i % 2 == 1) // filter by modulo
        .map(|(i, _)| i * i) // mapping
        .fold(0, |result, i| result + i); // collect into result: base var = 0 , old result + i
}

fn fallback(param: u8) -> u8 {
    42 + param
}
