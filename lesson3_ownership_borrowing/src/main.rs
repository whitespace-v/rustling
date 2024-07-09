fn main() {
    let a = 42;
    let b = a; // a copied

    //only for strings and tuple
    let s1 = String::from("hello");
    let s2: String = s1; // s1 moved

    //borrowing
    //demo_borrow_str(&s2);
    //println!("{s2}");

    //moving
    //demo_move_str(s2);
    //println!("{s2}"); //borrow of moved value: `s1`

    //slice

    let s2 = &s2[0..3];

    let str = "test"; // literal,
                      // ее не будет в хипе она ссылкается на какой-то кусок в исполняемом файле,
                      // это прямо слайс и есть, то есть блок памяти из бинарника,
                      // соответственно он не может быть изменяемым

    // в &str можно записать и &str и &String, а в &String только &String
}

fn demo_move_str(s: String) {
    println!("{s}")
}

fn demo_borrow_str(s: &String) {
    println!("{s}")
}
