use std::{ rc::Rc, sync::{ mpsc, Mutex , Arc}, thread::{ self }, time::Duration };

fn main() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from spawned thread");
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    // move to be able to use variables in threads
    thread::spawn(move || {
        for getal in v {
            println!("hi number {getal} from v in thread");
        }
    });

    // sending data from one thread to another

    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hallo "),
            String::from("ik "),
            String::from("ben "),
            String::from("Kasper "),
            String::from("Bosteels "),
            String::from("Bye.")
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("hallo "),
            String::from("ik "),
            String::from("ben "),
            String::from("een "),
            String::from("tweede "),
            String::from("thread.")
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(5));
        }
    });

    for received in rx {
        println!("{received}");
    }

    for i in 1..5 {
        println!("hi number {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }

    // Mutex shared states between threads

    let m = Mutex::new(5);
    // used synchonrioslously
    {
        let mut num = m.lock().unwrap();
        *num += 1;
    }
    println!("m = {:?}", m);

    // multiple threads and mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
