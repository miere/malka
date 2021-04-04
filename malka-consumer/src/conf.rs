use serde::Deserialize;
use std::collections::HashMap;
use rdkafka::ClientConfig;
use std::env;
use log::{info};
use rdkafka::config::RDKafkaLogLevel;

#[derive(Deserialize, Clone, Debug, PartialEq)]
pub struct SubscriptionConfig {
    pub topic_name: String,
    #[serde(default = "min_number_of_consumers")]
    pub topic_number_of_consumers: u32,
    #[serde(default = "max_buffer_size")]
    pub topic_max_buffer_size: usize,
    #[serde(default = "max_buffer_await_time_ms")]
    pub topic_max_buffer_await_time: u64,
    #[serde(default)]
    pub consumer_configuration: Option<HashMap<String, String>>,
    pub target_functions: Vec<String>
}

fn min_number_of_consumers() -> u32 { 1 }
fn max_buffer_size() -> usize { 100 }
fn max_buffer_await_time_ms() -> u64 { 1000 }

impl SubscriptionConfig {

    /// Creates a rdkafka::ClientConfig object based on this configuration.
    pub fn as_client_config_for(&self, target_function: &str, parallel_consumer_id: u32) -> ClientConfig {
        let group_id = format!("{}-{}", &self.topic_name, &target_function);
        info!("Consumer Group ID: {}", &group_id);

        let group_instance_id = format!("{}-{}-{}", &self.topic_name, target_function, parallel_consumer_id);
        info!("Consumer Group ID: {}", &group_instance_id);

        let mut config = SubscriptionConfig::create_default_kafka_config();
        config.set("group.id", group_id);
        config.set("group.instance.id", group_instance_id);
        config.set("fetch.wait.max.ms", &self.topic_max_buffer_await_time.to_string());
        config.set("batch.num.messages", &self.topic_max_buffer_size.to_string());

        if let Some(extra_config) = &self.consumer_configuration {
            for (key, value) in extra_config {
                config.set(key, value);
            }
        }

        config
    }

    fn create_default_kafka_config() -> ClientConfig {
        let mut cfg = ClientConfig::new();

        let kafka_brokers = env::var("KAFKA_BROKERS")
            .unwrap_or_else(|_| "127.0.0.1:9092".to_string());
        info!("Connecting to brokers: {}", &kafka_brokers);

        let security_protocol = env::var("KAFKA_SECURITY_PROTOCOL")
            .unwrap_or_else(|_| "plaintext".to_string());
        info!("Using security protocol: {}", &security_protocol);

        cfg.set("bootstrap.servers", kafka_brokers)
            .set("security.protocol", security_protocol)
            .set("enable.auto.commit", "false")
            .set_log_level(RDKafkaLogLevel::Debug);

        cfg
    }
}

#[cfg(test)]
mod test {
    use crate::conf::SubscriptionConfig;

    #[test]
    fn should_serialize_subscription_config_correctly() {
        let json = r#"[
         { "topic_name": "user.delete", "target_functions": ["user_deleted"] },
         { "topic_name": "user.update", "topic_number_of_consumers": 2, "target_functions": ["user_updated"] }
        ]"#;

        let configs: Vec<SubscriptionConfig> = serde_json::from_str(json).unwrap();
        assert_eq!(2, configs.len());

        let expected_first_cfg = SubscriptionConfig {
            topic_name: "user.delete".to_string(),
            topic_number_of_consumers: 1,
            topic_max_buffer_await_time: 1000,
            topic_max_buffer_size: 100,
            consumer_configuration: None,
            target_functions: vec!("user_deleted".to_string())
        };
        assert_eq!(expected_first_cfg, configs[0]);

        let expected_second_cfg = SubscriptionConfig {
            topic_name: "user.update".to_string(),
            topic_number_of_consumers: 2,
            topic_max_buffer_await_time: 1000,
            topic_max_buffer_size: 100,
            consumer_configuration: None,
            target_functions: vec!("user_updated".to_string())
        };
        assert_eq!(expected_second_cfg, configs[1]);
    }
}