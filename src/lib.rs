use std::{sync, thread};
// use std::time::Duration;

pub fn sleep_sort<I: Iterator<Item=u32>>(nums: I) -> Vec<u32> {
    let (tx, rx) = sync::mpsc::channel();

    let threads = nums.map(|n| {
        let tx = tx.clone();
        thread::spawn(move || {
            // thread::sleep(Duration::new(0, n*1000000));
            thread::sleep_ms(n);    // deprecated
            tx.send(n).unwrap();
        })
    }).collect::<Vec<_>>();

    rx.iter().take(threads.len()).collect::<Vec<_>>()
}
