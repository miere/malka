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
  source = "github.com/miere/malka-terraform-module?ref=v0.1.0"

  configuration = [{
    topic_name = "topic",
    topic_number_of_consumers = 2,
    consumer_configuration = {},
    target_functions = ["appetifyMailEngine"]
  }]

  kafka_brokers = ["localhost:9021"]

  region = "us-east-1"

  subnet_ids = data.aws_subnet_ids.default.ids
  vpc_id = data.aws_vpc.default.id
}
