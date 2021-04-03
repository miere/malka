use async_trait::async_trait;
use serde::{Serialize};

/// A Kafka Consumer wrapper. Created, basically, to leverage proper
/// unit testing when subscribing and consuming messages.
#[async_trait]
pub trait KafkaConsumer<LISTENER>
  where LISTENER: KafkaConsumerListener + std::marker::Sync {

    async fn consume(&self, listener: &LISTENER) -> KafkaConsumerResult;
}

/// Represents a consumer Kafka transaction.
#[async_trait]
pub trait KafkaConsumerTransaction {
    async fn commit(&self);
    async fn rollback(&self);
}

/// The resulting outcome of a message consumption.
#[derive(Debug,PartialEq,Clone)]
pub enum KafkaConsumerResult {
    Succeeded, Failed(String)
}

/// Defines a listener for the Kafka consumer.
#[async_trait]
pub trait KafkaConsumerListener {
    async fn consume(&self, record: Vec<InFlightRecord>) -> KafkaConsumerResult;
}

/// Represents an in-flight message.
#[derive(Serialize)]
pub struct InFlightRecord {
    pub key: Option<String>,
    pub value: Option<String>
}

impl InFlightRecord {

    pub fn create(key: Option<&[u8]>, value: Option<&[u8]>) -> Self {
        InFlightRecord {
            key: key.map(InFlightRecord::convert_bytes_to_str),
            value: value.map(InFlightRecord::convert_bytes_to_str)
        }
    }

    fn convert_bytes_to_str(bytes: &[u8]) -> String {
        std::str::from_utf8(bytes).unwrap().to_string()
    }
}

#[cfg(test)]
mod json_tests {
    use crate::kafka::consumer::InFlightRecord;

    const EXPECTED_KEY: &str = "my_key";
    const EXPECTED_VALUE: &str = "{\"hello\":\"world\"}";
    const EXPECTED_JSON: &str = "{\"key\":\"my_key\",\"value\":\"{\\\"hello\\\":\\\"world\\\"}\"}";

    #[test]
    fn should_be_able_to_serialize_in_flight_record_into_json_string() {
        let key = Some(EXPECTED_KEY.as_bytes());
        let value = Some(EXPECTED_VALUE.as_bytes());

        let record = InFlightRecord::create(key, value);
        let json_string = serde_json::to_string(&record).expect("Failed to serialize message");
        assert_eq!(EXPECTED_JSON, json_string)
    }
}

#[cfg(test)]
pub(crate) mod mocks {
    use std::sync::Arc;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::Ordering::Release;
    use super::*;
    use tokio::time::Instant;
    use std::ops::Add;
    use std::time::Duration;

    pub struct MockKafkaConsumerAndTransaction {
        consume_called: Arc<AtomicBool>,
        consume_expected_result: KafkaConsumerResult,
        rollback_called: Arc<AtomicBool>,
        commit_called: Arc<AtomicBool>,
    }

    impl MockKafkaConsumerAndTransaction {
        pub fn new() -> Self {
            MockKafkaConsumerAndTransaction {
                consume_called: Arc::new(Default::default()),
                consume_expected_result: KafkaConsumerResult::Succeeded,
                commit_called: Arc::new(Default::default()),
                rollback_called: Arc::new(Default::default()),
            }
        }

        pub fn reference_to_check_if_consumer_has_been_called(&self) -> Arc<AtomicBool> {
            Arc::clone(&self.consume_called)
        }
    }

    #[async_trait]
    impl<LISTENER> KafkaConsumer<LISTENER>
        for MockKafkaConsumerAndTransaction
        where LISTENER: KafkaConsumerListener + std::marker::Sync {

        async fn consume(&self, _listener: &LISTENER) -> KafkaConsumerResult {
            self.consume_called.store(true, Release);
            self.consume_expected_result.clone()
        }
    }

    #[async_trait]
    impl KafkaConsumerTransaction
        for MockKafkaConsumerAndTransaction {

        async fn commit(&self) {
            tokio::time::sleep_until(Instant::now().add(Duration::from_secs(1))).await;
            self.commit_called.store(true, Release);
        }

        async fn rollback(&self) {
            tokio::time::sleep(Duration::from_millis(500)).await;
            self.rollback_called.store(true, Release);
        }
    }

    pub struct MockKafkaConsumerListener {
        consume_called: Arc<AtomicBool>,
        consume_expected_result: KafkaConsumerResult
    }

    impl MockKafkaConsumerListener {

        pub fn new() -> MockKafkaConsumerListener {
            MockKafkaConsumerListener {
                consume_called: Arc::new(Default::default()),
                consume_expected_result: KafkaConsumerResult::Succeeded
            }
        }

        pub fn reference_to_check_if_consumer_has_been_called(&self) -> Arc<AtomicBool> {
            Arc::clone(&self.consume_called)
        }
    }

    #[async_trait]
    impl KafkaConsumerListener
    for MockKafkaConsumerListener {
        async fn consume(&self, _record: Vec<InFlightRecord>) -> KafkaConsumerResult {
            self.consume_called.store(true, Release);
            tokio::time::sleep(Duration::from_millis(100)).await;
            self.consume_expected_result.clone()
        }
    }
}

