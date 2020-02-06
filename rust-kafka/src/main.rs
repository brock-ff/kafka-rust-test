// run both a consumer and a producer (?)
mod consume;
mod produce;

use consume::consume;
use produce::produce;
use std::thread;

fn main() {
    println!("Hello, world!");
    thread::spawn(move || loop {
        thread::sleep_ms(1000);
        produce();
    });
    consume();
}
