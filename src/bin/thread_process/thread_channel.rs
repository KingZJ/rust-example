use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

const THREAD_NUM: i32 = 3;

pub fn channel_test() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    for id in 0..THREAD_NUM {
        let thread_tx = tx.clone();
        thread::spawn(move || {
            thread_tx.send(id).unwrap();

            println!("thread {} finished", id);
        });
    }

    let mut ids = vec![];
    for _ in 0..THREAD_NUM {
        ids.push(rx.recv());
    }

    println!("channel send sequence {:?}", ids);
}
