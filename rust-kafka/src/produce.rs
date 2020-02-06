extern crate rand;

use crate::{
    common::{get_host, TOPIC},
    data::{Data, Key},
    encode::encode,
};
use kafka::producer::{Producer, Record, RequiredAcks};
use std::time::Duration;

pub fn produce(key: Key) -> Result<(), serde_json::Error> {
    let mut producer = Producer::from_hosts(vec![get_host()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    let dat = Data::new();

    println!("producing message... id:{}", dat.id);

    let _ = producer.send(&Record::from_key_value(TOPIC, encode(&key)?, encode(&dat)?));
    Ok(())
}
