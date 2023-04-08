// Closure
// closure borrows values from the current scope
// that means thte closure can't outlive the scope
// to make the closure outlive the scope, we can use the move keyword
// the move keyword forces the closure to take ownership of the values it uses

// Thread
// thread::spawn() takes a closure and returns a JoinHandle
// the closure is the main function of the thread
// the handle.join() method waits for the thread to finish

use crossbeam::channel::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

fn expensive_sum(v: Vec<i32>) -> i32 {
    pause_ms(500);
    println!("Child thread: just about finished");
    v.iter().filter(|&n| n % 2 == 0).map(|n| n * n).sum()
}

fn pause_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

pub fn run() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];
    let handle = thread::spawn(move || expensive_sum(my_vector));

    for letter in &["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Letter {}", letter);
        pause_ms(200);
    }

    let sum = handle.join().unwrap();
    println!("The child thread's expensive sum is {}", sum);

    let (tx, rx) = channel::unbounded();
    let tx2 = tx.clone();

    let handle_a = thread::spawn(move || {
        pause_ms(0);
        tx2.send("Thread A: 1").unwrap();
        pause_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });

    pause_ms(100);

    let handle_b = thread::spawn(move || {
        pause_ms(0);
        tx.send("Thread B: 1").unwrap();
        pause_ms(200);
        tx.send("Thread B: 2").unwrap();
    });

    for msg in rx {
        println!("Main thread: Received {}", msg);
    }

    handle_a.join().unwrap();
    handle_b.join().unwrap();

    println!("Main thread: Exiting.");

    let (tx, rx): (Sender<i32>, Receiver<i32>) = channel::unbounded();
    let rx2 = rx.clone();
    let handle_a = thread::spawn(move || {
        for msg in rx2 {
            println!("Thread A: Received {}", msg);
        }
    });

    let handle_b = thread::spawn(move || {
        for msg in rx {
            println!("Thread B: Received {}", msg);
        }
    });
    vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10].iter().for_each(|n| {
        println!("Main thread: Sending {}", n);
        tx.send(*n).unwrap()
    });
    drop(tx);
    handle_a.join().unwrap();
    handle_b.join().unwrap();
}
