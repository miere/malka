use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Acquire;

use log::{debug, error, trace};

use KafkaConsumerResult::{Failed, Succeeded};

use crate::kafka::consumer::{KafkaConsumer, KafkaConsumerListener, KafkaConsumerResult, KafkaConsumerTransaction};

/// A simplified Kafka subscriber. It wraps away the complexity of
/// dealing with transactions when consuming messages.
pub struct KafkaSubscriber<CONSUMER, LISTENER>
  where CONSUMER: KafkaConsumer<LISTENER> + KafkaConsumerTransaction,
        LISTENER: KafkaConsumerListener + std::marker::Sync
{
    pub should_poll_next_messages: Arc<AtomicBool>,
    pub consumer: CONSUMER,
    pub listener: LISTENER
}

impl<CONSUMER, LISTENER> KafkaSubscriber<CONSUMER, LISTENER>
    where CONSUMER: KafkaConsumer<LISTENER> + KafkaConsumerTransaction,
          LISTENER: KafkaConsumerListener + std::marker::Sync {

    /// Performs the message consumption loop.
    /// The loop will be interrupted once `should_poll_next_messages` is set to `false`.
    pub async fn main_loop(&self) {
        while self.should_poll_next_messages.load(Acquire) {
            trace!("Polling messages...");
            let result = self.consumer.consume(&self.listener).await;
            match result {
                Failed(cause) => self.rollback(cause).await,
                Succeeded => self.commit().await
            }
        }
    }

    /// Commits the transaction and moves the cursor forward once
    /// the message was correctly ingested.
    async fn commit(&self) {
        debug!("Messages has been consumed");
        self.consumer.commit().await;
    }

    /// Performs a rollback in the last execution in case of failure.
    /// It's expected, though, only failures related to network communication
    /// to be handled here. The message will be considered delivered even if
    /// the Lambda function failed to handle the event. Dead-Letter Queues,
    /// Backoff and Retries are capabilities provided out-of-box from AWS Lambda,
    /// therefore doesn't need to be implemented in this application.
    async fn rollback(&self, cause: String) {
        error!("Failed to consume message: {}", cause);
        self.consumer.rollback().await;
    }
}

#[cfg(test)]
mod kafka_subscriber_tests {

    #[cfg(test)]
    mod when_running_main_loop {
        use std::sync::Arc;
        use std::sync::atomic::{AtomicBool, Ordering::*};
        use std::time::Duration;

        use crate::kafka::consumer::mocks::MockKafkaConsumerAndTransaction as MockKafkaConsumer;
        use crate::kafka::consumer::mocks::MockKafkaConsumerListener;
        use crate::kafka::subscriber::KafkaSubscriber;

        #[tokio::test]
        async fn should_invoke_consumer_correctly_if_allowed_to_poll_messages() {
            let should_poll_messages = Arc::new(AtomicBool::new(true));

            let listener = MockKafkaConsumerListener::new();
            let consumer = MockKafkaConsumer::new();
            let consumer_called = consumer.reference_to_check_if_consumer_has_been_called();

            let subscriber = KafkaSubscriber {
                should_poll_next_messages: Arc::clone(&should_poll_messages),
                consumer, listener
            };

            let future = tokio::spawn(async move {
                subscriber.main_loop().await;
            });

            tokio::time::sleep(Duration::from_millis(500)).await;
            should_poll_messages.store(false, Relaxed);

            future.await.expect("Failed to shutdown thread");
            assert!(consumer_called.load(Relaxed));
        }
    }
}
