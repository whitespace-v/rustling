use std::{
    cell::{Ref, RefCell},
    ops::Deref,
};
fn main() {
    let a = 5; // stack
    let b = &a; // heap

    // a == b cant eq  &i32 and i32
    a == *b; // eq by deref

    let mut c = *b; // copy of b

    let av = vec![1, 2, 3]; //heap (vectors and Strings -> smart pointers)

    let ab = &av; // not smart pointer -> just pointer

    // let ac = *ab;// error
    let ac = (*b).clone(); // different vector
    {
        let a: Vec<i32> = vec![1, 2, 3];
        // let b = *a; // array
    }
    {
        //  only for String and vectors
        let a = String::from("hello");
        let b = a.deref();

        let c = vec![1, 2, 3];
        let d = c.deref(); //
    }
    {
        let a = "test";
        let b = String::from(a);
        call(a);
        call(&b);
    }
    {
        let a = 42; // stack
        let a = Box::new(42); // heap
        let b: &Box<i32> = &a;
        let c: &i32 = a.deref();
        let d: i32 = *a; // if change d -> a not changing
    }
    {
        struct Cat {
            name: String,
            age: u8,
        }
        // all cat in stack,
        // name pointer -> stack
        // name -> stack
        let cat: Cat = Cat {
            name: String::from("Mister"),
            age: 21,
        };
    }
    {
        struct Cat {
            name: String,
            age: u8,
            parent: Option<Box<Cat>>,
        }
        // size is unknown, so it will be in heap

        let cat_parent = Box::new(Cat {
            name: String::from("Mister"),
            age: 21,
            parent: None,
        });
        let cat: Cat = Cat {
            name: String::from("Mister"),
            age: 21,
            parent: Some(cat_parent), //move
        };
    }

    {
        struct MyBox<T>(T);
        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }
        // trait
        impl<T> Deref for MyBox<T> {
            type Target = T;
            fn deref(&self) -> &Self::Target {
                &self.0 // bound to x
            }
        }
    }
    // drop trait
    {
        struct CustomSmartPointer {
            data: String,
        }
        impl Drop for CustomSmartPointer {
            fn drop(&mut self) {
                println!("Drop customsmartpointer with data: `{}`", self.data)
            }
        }
        //  force drop
        let c = CustomSmartPointer {
            data: String::from("st"),
        };
        let b = CustomSmartPointer {
            data: String::from("st"),
        };
        std::mem::drop(c)
    }
    {
        // reference counting -- RC много переменных могут иметь одинаковые указатели -
        // данные удаляться тогда когда последний поинтер удалится
        use std::rc::Rc;
        struct Cat {
            name: String,
            age: u8,
            parent: Option<Rc<Cat>>,
        };
        let cat_parent = Rc::new(Cat {
            name: String::from("Mister"),
            age: 21,
            parent: None,
        });
        println!("{}", Rc::strong_count(&cat_parent)); // 1
        let cat: Cat = Cat {
            name: String::from("Mister"),
            age: 21,
            parent: Some(Rc::clone(&cat_parent)), // copy only pointer
        };
        println!("{}", Rc::strong_count(&cat_parent)); // 2
    }
    {
        // RefCell
        // interior mutability
        use std::rc::Rc;
        let a = RefCell::new(5);
        let b = &a;
        *a.borrow_mut() += 1; //mutability without 'mut' oncely borrowed
    }
    {
        // combine rfcell and rc
        use std::cell::RefCell;
        use std::rc::Rc;

        struct Cat {
            name: String,
            age: u8,
            parent: Option<Rc<RefCell<Cat>>>,
        }

        let cat_elder = Rc::new(RefCell::new(Cat {
            name: String::from("Elder"),
            age: 16,
            parent: None,
        }));

        let cat_parent = Rc::new(RefCell::new(Cat {
            name: String::from("Parent"),
            age: 4,
            parent: Some(Rc::clone(&cat_elder)),
        }));

        let cat = Rc::new(RefCell::new(Cat {
            name: String::from("Mr. Buttons"),
            age: 2,
            parent: Some(Rc::clone(&cat_parent)),
        }));

        cat_parent.borrow_mut().age = 12; // change data of `cat_parent` in all recursive objects

        //so we can change data from children
        cat.borrow().parent.as_ref().unwrap().borrow_mut().age = 8;

        parents_iterate(&cat);

        fn parents_iterate(child: &Rc<RefCell<Cat>>) {
            let mut current = Some(Rc::clone(child));

            while let Some(current_cat) = current.take() {
                let current_cat_ref = current_cat.borrow();
                println!(
                    "Getting cat named {}, age {}",
                    current_cat_ref.name, current_cat_ref.age
                );
                if let Some(parent) = current_cat_ref.parent.as_ref() {
                    let parent_ref = parent.borrow();
                    println!(
                        "{}'s parent is {} and its age is {}",
                        current_cat_ref.name, parent_ref.name, parent_ref.age
                    );
                    current = Some(Rc::clone(parent))
                }
                println!("============")
            }
        }
    }
}

fn call(s: &str) {
    println!("{s}")
}
