use std::sync::{mpsc, Arc};
use std::thread;

mod file_path;
mod thread_channel;
fn main() {
    let (tx, rx) = mpsc::channel();
    tx.send(0);
    loop {
        println!("loop starting");
        println!("{:?}", rx.recv());
        println!("loop end and start next");
    }
    // arc_test();
    // map_reduce_test();

    thread_channel::channel_test();

    file_path::path_test();
}

fn arc_test() {
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        let apple = Arc::clone(&apple);

        thread::spawn(move || {
            println!("{:?}", apple);
        });
    }
}

fn map_reduce_test() {
    let data = "890987657
1109876
8726522
9987652";

    let mut children = vec![];
    let chunked_data = data.split_whitespace();

    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is {}", i, data_segment);

        children.push(thread::spawn(move || -> u32 {
            let res = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("need a digit"))
                .sum();
            println!("processed segment {}, res={}", i, res);

            res
        }));
    }

    let mut temp_sums = vec![];
    for child in children {
        temp_sums.push(child.join().unwrap());
    }

    println!("final sum result: {}", temp_sums.iter().sum::<u32>());
}
