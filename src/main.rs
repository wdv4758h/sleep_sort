extern crate sleep;

use sleep::sleep_sort;

fn main() {
    // ./target/release/sleepsort 0 9 10 3 5 7 18 78 1 2 100 100 1 2 1 2 1 800
    let result = sleep_sort(std::env::args().skip(1)
                                            .map(|s| s.parse().unwrap()));
    println!("{:?}", result);
}
