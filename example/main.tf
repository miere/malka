# Configure AWS provider
provider "aws" {
  version = "~> 2.0"
  region = "us-east-1"
}

# Fetch minimal data from your AWS services
data "aws_vpc" "default" {
  default = true
}

data "aws_subnet_ids" "default" {
  vpc_id = data.aws_vpc.default.id
}

# Configure Malka
module "malka" {
  source  = "miere/malka/aws"
  version = "0.2.0"

  configuration = [{
    topic_name = "topic",
    topic_number_of_consumers = 2,
    consumer_configuration = {},
    target_functions = ["my_lambda_function"]
  }]

  kafka_brokers = ["localhost:9021"]
  kafka_security_protocol = "plaintext"

  subnet_ids = data.aws_subnet_ids.default.ids
  vpc_id = data.aws_vpc.default.id
}
