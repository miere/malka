use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use log::{info, trace};

use crate::aws::lambda_publisher::AwsLambdaKafkaConsumerListener;
use crate::conf::SubscriptionConfig;
use crate::kafka::defaults::DefaultKafkaConsumer;
use crate::kafka::subscriber::KafkaSubscriber;
use crate::error::Result;
use std::sync::atomic::Ordering::Release;
use tokio::task::JoinHandle;

type DefaultKafkaSubscriber = KafkaSubscriber<DefaultKafkaConsumer, AwsLambdaKafkaConsumerListener>;
type SubscriberEnabledFlag = Arc<AtomicBool>;
type SubscribersRef = HashMap<String, SubscriberEnabledFlag>;

pub struct SubscriptionManager {
    subscribers: SubscribersRef,
    subscribers_thread_future: Vec<JoinHandle<()>>
}

impl Default for SubscriptionManager {
    fn default() -> Self {
        SubscriptionManager {
            subscribers: HashMap::new(),
            subscribers_thread_future: Vec::new()
        }
    }
}

impl SubscriptionManager {

    /// Subscribe to a give `topic subscription configuration`.
    pub fn subscribe(&mut self, subscription: SubscriptionConfig) -> Result<()> {
        for target_function in subscription.target_functions.iter() {
            for parallel_consumer_id in 0..subscription.topic_number_of_consumers {
                self.subscribe_to_function(&subscription, target_function, parallel_consumer_id)?;
            }
        }

        Ok(())
    }

    fn subscribe_to_function(&mut self, subscription: &SubscriptionConfig, target_function: &str, parallel_consumer_id: u32) -> Result<()> {
        let subscriber = SubscriptionManager::create_subscriber_from(
            subscription, target_function, parallel_consumer_id)?;
        let flag = Arc::clone(&subscriber.should_poll_next_messages);

        let future = tokio::spawn(async move {
            subscriber.main_loop().await
        });

        self.subscribers.insert(subscription.topic_name.clone(), flag);
        self.subscribers_thread_future.push(future);

        Ok(())
    }

    fn create_subscriber_from(subscription: &SubscriptionConfig, target_function: &str, parallel_consumer_id: u32) -> Result<DefaultKafkaSubscriber>
    {
        let config = subscription.as_client_config_for(target_function, parallel_consumer_id);
        let group_instance_id = config.get("group.instance.id").unwrap();
        let listener = AwsLambdaKafkaConsumerListener::create(target_function.to_string());
        let should_poll_next_messages = Arc::new(AtomicBool::new(true));
        let consumer = DefaultKafkaConsumer::create(
            group_instance_id.to_string(),
            subscription.topic_name.to_string(),
            subscription.topic_max_buffer_size,
            subscription.topic_max_buffer_await_time,
            config)?;

        Ok(KafkaSubscriber {
            should_poll_next_messages,
            consumer, listener
        })
    }

    pub async fn await_termination(mut self) -> Result<()> {
        futures::future::join_all(
            &mut self.subscribers_thread_future
        ).await;
        Ok(())
    }
}

impl Drop for SubscriptionManager {

    fn drop(&mut self) {
        let subscribers = &mut self.subscribers;

        for (topic_name, flag) in subscribers.iter_mut() {
            info!("Unsubscribing to topic {}", &topic_name);
            flag.store(false, Release);
            trace!("Successfully unsubscribed to topic {}", topic_name)
        }
    }
}
