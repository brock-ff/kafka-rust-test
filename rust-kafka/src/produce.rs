extern crate rand;

use crate::encode::encode;
use kafka::producer::{Producer, Record, RequiredAcks};
use rand::Rng;
use std::time::Duration;

pub fn produce() {
    let mut producer = Producer::from_hosts(vec!["0.0.0.0:9092".to_owned()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    // get random number
    let mut rng = rand::thread_rng();
    let v: u8 = rng.gen();
    println!("v: {}", v);

    println!("producing message...");

    let mut buf = String::with_capacity(2);
    let val = encode(v, &mut buf);
    producer.send(&Record::from_value("test", val)).unwrap();
    buf.clear();
}
