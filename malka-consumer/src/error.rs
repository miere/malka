use thiserror::Error;
use rdkafka::error::KafkaError;

pub type Result<T> = std::result::Result<T, KnownHandledErrors>;

#[derive(Error, Debug)]
pub enum KnownHandledErrors {

    #[error(transparent)]
    Kafka(#[from] KafkaError),

    #[error("Expected one or more 'file names' as parameters")]
    InvalidParameters
}