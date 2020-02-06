extern crate rand;

use crate::{
    common::{get_host, TOPIC},
    data::Data,
    encode::encode,
};
use kafka::producer::{Producer, Record, RequiredAcks};
use std::time::Duration;

pub fn produce() {
    let mut producer = Producer::from_hosts(vec![get_host()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    let d = Data::new();

    println!("producing message... id:{}", d.id);

    let val = encode(&d);
    producer.send(&Record::from_value(TOPIC, val)).unwrap();
}
