use std::{f32::consts::PI, rc::Rc, sync::Mutex, thread, time::Duration};

fn main() {
    // {
    //     let a = 42;

    //     let v = vec![1, 2, 3];

    //     let handle = thread::spawn(move || {
    //         for i in 1..10 {
    //             println!("{v:?} {} from spawned", i);
    //             thread::sleep(Duration::from_millis(1));
    //         }
    //     });

    //     for i in 1..5 {
    //         println!("{} from thread", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }

    //     // wait till handle worked
    //     handle.join().unwrap();
    // }
    // {
    //     // multiple producer single consumer // mpsc

    //     // transmitter, reciever
    //     use std::sync::mpsc;
    //     let (tx, rx) = mpsc::channel();

    //     thread::spawn(move || {
    //         let val = String::from("hi");
    //         // send to transmitter String
    //         tx.send(val).unwrap();
    //     });

    //     let recieved = rx.recv().unwrap();
    //     println!("{recieved}")
    // }

    // many threads with 1 channel (single consumer)
    // {
    //     use std::sync::mpsc;
    //     let (tx, rx) = mpsc::channel();

    //     let tx1 = tx.clone();

    //     thread::spawn(move || {
    //         let vals = vec![
    //             String::from("hi"),
    //             String::from("123123"),
    //             String::from("sdfsdfsdf"),
    //             String::from("dfgdfh"),
    //             String::from("dasf"),
    //         ];

    //         for val in vals {
    //             tx.send(val).unwrap();
    //             thread::sleep(Duration::from_millis(1));
    //         }
    //     });

    //     thread::spawn(move || {
    //         let vals = vec![
    //             String::from("2hi"),
    //             String::from("2123123"),
    //             String::from("2sdfsdfsdf"),
    //             String::from("2dfgdfh"),
    //             String::from("2dasf"),
    //         ];

    //         for val in vals {
    //             tx1.send(val).unwrap();
    //             thread::sleep(Duration::from_millis(1));
    //         }
    //     });

    //     for recieved in rx {
    //         println!("{recieved}")
    //     }
    // }

    //mutex
    // {
    //     use ::std::sync::Mutex;

    //     let m = Mutex::new(5);
    //     {
    //         let mut num = m.lock().unwrap();
    //         *num = 6;
    //     }
    // }

    {
        // many mutable borrows
        use std::sync::{Arc, Mutex};

        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);

            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
                println!("{num}")
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }
}
