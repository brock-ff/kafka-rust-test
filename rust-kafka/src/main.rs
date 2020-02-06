// run both a consumer and a producer (?)
mod common;
mod consume;
mod data;
mod encode;
mod produce;

use consume::do_consume;
use data::{Data, Key};
use produce::produce;
use std::thread;

fn main() -> Result<(), std::io::Error> {
    println!("Hello, world!");
    thread::spawn(move || loop {
        thread::sleep(std::time::Duration::from_millis(1000));
        produce(Key::MintErc20, Data::new())
            .map_err(|_e| std::io::Error::from(std::io::ErrorKind::InvalidInput))
            .unwrap_or(());
    });
    do_consume();
    Ok(())
}
