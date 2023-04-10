// Single receiver, multiple sender
use std::thread;
use std::sync::{Arc, mpsc};

fn square(v: Arc<Vec<i32>>, tx: mpsc::Sender<i32>) -> thread::JoinHandle<()>{
    return thread::spawn(move || {
        for n in v.iter() {
            tx.send(n * n).unwrap();
        }
    });
}

fn factorial(v: Arc<Vec<i32>>, tx: mpsc::Sender<i32>) -> thread::JoinHandle<()>{
    return thread::spawn(move || {
        for n in v.iter() {
            let mut f:i32 = 1;
            let mut n1:i32 = *n;
            while n1 > 0 {
                f = f * n1;
                n1 = n1 - 1;
            }
            tx.send(f).unwrap();
        }
    });
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let v = Arc::new(vec![1, 2, 3]);

    let tx1 = tx.clone();
    factorial(v.clone(), tx);
    square(v.clone(), tx1);

    for n in rx {
        println!("{n}");
    }
    println!("cleanup");
}
