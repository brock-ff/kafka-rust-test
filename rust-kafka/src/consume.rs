use crate::{
    common::{get_host, TOPIC},
    data::{Data, Key},
    encode::decode,
};
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};
use std::thread;

fn consume(consumer: &mut Consumer) -> Result<(), serde_json::Error> {
    for ms in consumer.poll().unwrap().iter() {
        for m in ms.messages() {
            let key: Key = decode(m.key)?;
            let dat: Data = decode(m.value)?;
            println!("{:?}", m);
            println!("consumed {}:\n{}", key, dat);
        }
        let _ = consumer.consume_messageset(ms);
    }
    consumer.commit_consumed().unwrap();
    println!("queue is empty...");
    Ok(())
}

pub fn do_consume() {
    let mut consumer = Consumer::from_hosts(vec![get_host()])
        // .with_topic_partitions("test".to_owned(), &[0, 1])
        .with_topic(TOPIC.to_owned())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group("my-group".to_owned())
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();

    loop {
        let _ = consume(&mut consumer).map_err(|e| {
            println!("Tried to consume invalid data: {}", e);
        });
        thread::sleep(std::time::Duration::from_millis(200));
    }
}
