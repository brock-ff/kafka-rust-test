// run both a consumer and a producer (?)
mod common;
mod consume;
mod data;
mod encode;
mod produce;

use consume::consume;
use produce::produce;
use std::thread;

fn main() {
    println!("Hello, world!");
    thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_millis(1000));
        produce();
    });
    consume();
}
