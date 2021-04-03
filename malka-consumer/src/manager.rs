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

    pub fn subscribe(&mut self, subscription: SubscriptionConfig) -> Result<()> {
        for target_function in subscription.target_functions.iter() {
            let subscriber = SubscriptionManager::create_subscriber_from(&subscription, target_function)?;
            let flag = Arc::clone(&subscriber.should_poll_next_messages);

            let future = tokio::spawn(async move {
                subscriber.main_loop().await
            });

            self.subscribers.insert(subscription.topic_name.clone(), flag);
            // let pinned_future = Box::pin();
            self.subscribers_thread_future.push(future);
        }

        Ok(())
    }

    fn create_subscriber_from(subscription: &SubscriptionConfig, target_function: &str) -> Result<DefaultKafkaSubscriber>
    {
        let config = subscription.as_client_config_for(target_function);
        let listener = AwsLambdaKafkaConsumerListener::create(target_function.to_string());
        let should_poll_next_messages = Arc::new(AtomicBool::new(true));
        let consumer = DefaultKafkaConsumer::create(
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

        for (topic_name, flag) in subscribers.into_iter() {
            info!("Unsubscribing to topic {}", &topic_name);
            flag.store(false, Release);
            trace!("Successfully unsubscribed to topic {}", topic_name)
        }
    }
}
