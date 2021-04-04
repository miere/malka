use std::time::{Duration, Instant};

use async_trait::async_trait;
use rdkafka::{ClientConfig, Message, TopicPartitionList};
use rdkafka::consumer::{CommitMode, Consumer, DefaultConsumerContext, BaseConsumer};
use rdkafka::message::BorrowedMessage;
use rdkafka::util::Timeout;
use log::{debug, trace};

use crate::error::Result;
use crate::kafka::consumer::{InFlightRecord, KafkaConsumer, KafkaConsumerListener, KafkaConsumerResult, KafkaConsumerTransaction};

const MSG_FAIL_TO_POLL: &str = "Could not poll messages.";
const MSG_FAIL_TO_COMMIT: &str = "Could not commit message. Interrupting this consumer to avoid data loss.";
const MSG_FAIL_TO_ROLLBACK: &str = "Could not rollback. Interrupting this consumer to avoid data loss.";

const TIMEOUT: Duration = Duration::from_secs(30);
const KAFKA_TIMEOUT: Timeout = Timeout::After(TIMEOUT);

/// The default KafkaConsumer implementation. It wraps away
/// the complexity of consuming message using `rdkafka`.
pub struct DefaultKafkaConsumer {
    stream_consumer: BaseConsumer<DefaultConsumerContext>,
    max_buffer_size: usize,
    max_buffer_await_time: Duration,
    group_instance_id: String,
}

impl DefaultKafkaConsumer {
    pub fn create(
        group_instance_id: String,
        topic_name: String,
        max_buffer_size: usize,
        max_buffer_await_time_millis: u64,
        cfg: ClientConfig
    ) -> Result<Self> {
        let context = DefaultConsumerContext {};
        let stream_consumer: BaseConsumer<DefaultConsumerContext> = cfg.create_with_context(context)?;
        stream_consumer.subscribe(&[&topic_name])?;

        Ok(DefaultKafkaConsumer {
            group_instance_id,
            stream_consumer,
            max_buffer_await_time: Duration::from_millis(max_buffer_await_time_millis),
            max_buffer_size
        })
    }

    fn read_received_message(&self, msg: &BorrowedMessage) -> InFlightRecord {
        InFlightRecord::create(
            msg.key(),
            msg.payload()
        )
    }

    async fn consume_and_buffer_messages(&self) -> Result<Vec<InFlightRecord>> {
        let mut buffer = Vec::new();
        let start = Instant::now();
        let mut elapsed = start.elapsed();
        while elapsed <= self.max_buffer_await_time && buffer.len() < self.max_buffer_size {
            trace!("[{}] Buffering messages...", &self.group_instance_id);
            let optional_message = self.stream_consumer.poll(self.max_buffer_await_time);
            if let Some(result) = optional_message {
                let message = result?;
                let record = self.read_received_message(&message);
                buffer.push(record);
            }

            elapsed = start.elapsed();
        }
        Ok(buffer)
    }
}

#[async_trait]
impl<LISTENER> KafkaConsumer<LISTENER> for DefaultKafkaConsumer
    where LISTENER: KafkaConsumerListener + std::marker::Sync {

    async fn consume(&self, listener: &LISTENER) -> KafkaConsumerResult {
        match self.consume_and_buffer_messages().await {
            Ok(received_message) if received_message.is_empty() => {
                debug!("[{}] No messages received.", &self.group_instance_id);
                KafkaConsumerResult::NoMessagesConsumed
            },
            Ok(received_message) => {
                debug!("[{}] Consuming {} message(s)", &self.group_instance_id, received_message.len());
                let result = listener.consume(received_message).await;
                result
            },
            Err(failure) => {
                let msg = format!("[{}] {}. \nDetails: {:?}", &self.group_instance_id, MSG_FAIL_TO_POLL, failure);
                KafkaConsumerResult::Failed(msg)
            }
        }
    }
}

#[async_trait]
impl KafkaConsumerTransaction
 for DefaultKafkaConsumer {

    async fn commit(&self) {
        let result = self.stream_consumer.commit_consumer_state(CommitMode::Sync);
        if let Err(cause) = result {
            panic!("[{}] {}. \nDetails: {:?}", &self.group_instance_id, MSG_FAIL_TO_COMMIT, cause)
        }
    }

    async fn rollback(&self) {
        let committed: TopicPartitionList = self.stream_consumer.committed(KAFKA_TIMEOUT)
            .expect(MSG_FAIL_TO_ROLLBACK);

        for ((topic,partition), offset) in committed.to_topic_map() {
            self.stream_consumer.seek(&topic, partition, offset, KAFKA_TIMEOUT)
                .expect(MSG_FAIL_TO_ROLLBACK)
        }
    }
}

#[cfg(test)]
mod integration_tests {
    use std::sync::atomic::Ordering::Relaxed;

    use rdkafka::ClientConfig;
    use rdkafka::config::RDKafkaLogLevel;
    use rdkafka::producer::{BaseProducer, BaseRecord, DefaultProducerContext};

    use crate::kafka::consumer::{KafkaConsumer, KafkaConsumerResult};
    use crate::kafka::consumer::mocks::MockKafkaConsumerListener;
    use crate::kafka::defaults::DefaultKafkaConsumer;

    #[tokio::test]
    #[ignore]
    async fn should_relay_in_flight_msg_to_the_consumer() {
        env_logger::init();

        let mut config = create_kafka_config();
        let producer: BaseProducer = config.create_with_context( DefaultProducerContext{} ).unwrap();

        let record = BaseRecord {
            topic: "test",
            key: Some("key"),
            payload: Some("{\"say\":\"hello\"}"),
            timestamp: None,
            headers: None,
            partition: None,
            delivery_opaque: ()
        };
        producer.send(record).unwrap();

        let listener = MockKafkaConsumerListener::new();
        let listener_called = listener.reference_to_check_if_consumer_has_been_called();

        config
            .set("auto.offset.reset", "earliest")
            .set("fetch.wait.max.ms", "100")
            .set("batch.num.messages", "1");
        let consumer = DefaultKafkaConsumer::create(
            "group_id_instance".to_string(),
            "test".to_string(),
            1, 100,
            config).unwrap();
        let result = consumer.consume(&listener).await;

        assert_eq!(KafkaConsumerResult::Succeeded, result);
        assert!(listener_called.load(Relaxed));
    }

    fn create_kafka_config() -> ClientConfig {
        let mut cfg = ClientConfig::new();

        cfg.set("group.id", "unit-test")
            .set("bootstrap.servers", "127.0.0.1:9092")
            .set("auto.commit.enable", "false")
            .set_log_level(RDKafkaLogLevel::Debug);

        return cfg;
    }
}
