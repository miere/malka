use async_trait::async_trait;
use bytes::Bytes;
use log::error;
use rusoto_core::Region;
use rusoto_lambda::{InvocationRequest, Lambda, LambdaClient};

use crate::kafka::consumer::{
    InFlightRecord, KafkaConsumerListener, KafkaConsumerResult
};

/// A `KafkaConsumerListener` implementation that invokes AWS Lambda functions.
pub struct AwsLambdaKafkaConsumerListener {
    function_name: String,
    lambda_client: LambdaClient
}

impl AwsLambdaKafkaConsumerListener {
    pub fn create(function_name: String) -> Self {
        let region: Region = Default::default();
        AwsLambdaKafkaConsumerListener {
            lambda_client: LambdaClient::new(region),
            function_name
        }
    }
}

#[async_trait]
impl KafkaConsumerListener
 for AwsLambdaKafkaConsumerListener {

    async fn consume(&self, records: Vec<InFlightRecord>) -> KafkaConsumerResult {
        let json_string = serde_json::to_string(&records).expect("Failed to serialize message");
        let json_bytes = Bytes::from(json_string);
        let result = self.lambda_client.invoke(InvocationRequest {
            function_name: self.function_name.clone(),
            payload: Some(json_bytes),
            client_context: None,
            invocation_type: None,
            log_type: None,
            qualifier: None
        }).await;

        match result {
            Ok(response) => {
                if let Some(error_msg) = response.function_error {
                    error!("Lambda function failed to handle request: {}", error_msg);
                }
                KafkaConsumerResult::Succeeded
            },
            Err(cause) => {
                let msg = format!("Failed to invoke function {}: {}", &self.function_name, cause);
                KafkaConsumerResult::Failed(msg)
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[tokio::test]
    #[ignore]
    async fn should_invoke_lambda() {
        let key = "k".as_bytes();
        let value = "{'hello':'world'}".as_bytes();
        let record = InFlightRecord::create(Some(key),Some(value));

        let consumer = AwsLambdaKafkaConsumerListener::create("user_deleted".to_string());
        let result = consumer.consume(vec!(record)).await;
        println!("Result: {:?}", result)
    }
}