
use std::time::Duration;
use anyhow::{Result, Context};
use rdkafka::{consumer::{BaseConsumer, Consumer}, ClientConfig, TopicPartitionList, Offset};

pub fn get_offset_for_time_rfc3339(
    brokers: &str,
    topic: &str,
    time_str:&str,
    timeout: Option<Duration>,
) -> Result<TopicPartitionList> {

    let timestamp = chrono::DateTime::parse_from_rfc3339(time_str)
        .with_context(|| format!("invalid time format [{}]", time_str))?
        .timestamp_millis();

    get_offset_for_timestamp(
        brokers, 
        topic, 
        timestamp,
        timeout,
    ).with_context(||format!("get_offset_for_timestamp fail, timestamp {}", timestamp))
}

pub fn get_offset_for_timestamp (
    brokers: &str,
    topic: &str,
    timestamp: i64,
    timeout: Option<Duration>,
) -> Result<TopicPartitionList> {

    let consumer: BaseConsumer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .create()
        .with_context(|| "creation failed: {:?}")?;

    let metadata = consumer
        .fetch_metadata(Some(topic), timeout.clone())
        .with_context(|| "fetch_metadata fail")?;

    let mtopic = metadata.topics()
        .first()
        .with_context(|| "fetch_metadata return empty")?;

    let mut tpl = TopicPartitionList::new();
    for partition in mtopic.partitions() {
        tpl.add_partition_offset(topic, partition.id(), Offset::Offset(timestamp))
            .with_context(|| "add_partition_offset fail")?;
    }

    let offset_tpl = consumer.offsets_for_times(tpl, None)
        .with_context(|| "offsets_for_times fail")?;

    Ok(offset_tpl)
}


#[derive(Debug)]
pub struct PartionOffset {
    pub partition: i32,
    pub offset: Offset,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_offset_for_time() {

        let time_str = "2023-06-15T10:11:12+08:00";

        let offset_tpl = get_offset_for_time_rfc3339(
            "127.0.0.1:30001", 
            "threeq-uplink-messages1", 
            time_str,
            None,
        ).unwrap();
    
        {
            println!("offsets for time {}", time_str);
            for ele in offset_tpl.elements().iter() {
                println!("  partition[{:02}] = {:?}", ele.partition(), ele.offset());
            }
        }
    }
}