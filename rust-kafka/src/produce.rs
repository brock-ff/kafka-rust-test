extern crate rand;

use crate::{
    common::{get_host, TOPIC},
    data::Key,
    encode::encode,
};
use kafka::producer::{Producer, Record, RequiredAcks};
use serde::Serialize;
use std::time::Duration;

pub fn produce<T: Serialize>(key: Key, data: T) -> Result<(), serde_json::Error> {
    let mut producer = Producer::from_hosts(vec![get_host()])
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .unwrap();

    let _ = producer.send(&Record::from_key_value(
        TOPIC,
        encode(&key)?,
        encode(&data)?,
    ));
    Ok(())
}
