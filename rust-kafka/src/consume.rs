use crate::encode::decode;
use kafka::consumer::{Consumer, FetchOffset, GroupOffsetStorage};

pub fn consume() {
    let mut consumer = Consumer::from_hosts(vec!["0.0.0.0:9092".to_owned()])
        // .with_topic_partitions("test".to_owned(), &[0, 1])
        .with_topic("test".to_owned())
        .with_fallback_offset(FetchOffset::Earliest)
        .with_group("my-group".to_owned())
        .with_offset_storage(GroupOffsetStorage::Kafka)
        .create()
        .unwrap();
    loop {
        for ms in consumer.poll().unwrap().iter() {
            for m in ms.messages() {
                println!("{:?}", m);
                println!("{:?}", decode(m.value));
            }
            let _ = consumer.consume_messageset(ms);
        }
        consumer.commit_consumed().unwrap();
        println!("queue is empty...");
    }
}
