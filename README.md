# malka
A service that leverages AWS Lambda as first-class Kafka consumers.

## Documentation
The main documentation is available in [the Wiki pages](https://github.com/miere/malka/wiki/) of this
repository. Here are most important topics:
- [Deploying Malka in your AWS infrastructure](https://github.com/miere/malka/wiki/Deploying-Malka-in-your-AWS)
- [AWS Deployment Infrastructure Overview](https://github.com/miere/malka/wiki/AWS-Deployment-Infrastructure)
- [Docker Container Architecture Overview](https://github.com/miere/malka/wiki/Docker-Container-Architecture)


## Usage
Malka is containerised and publicaly available [for download](https://hub.docker.com/r/miere/malka-consumer)
as a Docker image. You can use this image to deploy it on your favourite container orchestration
platform (e.g. ECS, BeanStalk, K8s, etc).

The easiest (but a bit opinionated) method to deploy it would be using Terraform.

```terraform
module "malka" {
  source = "github.com/miere/malka-terraform-module?ref=v0.1.0"
  
  # AWS Region
  region = "us-east-1"
  
  # AWS network configuration
  subnet_ids = data.aws_subnet_ids.default.ids
  vpc_id = data.aws_vpc.default.id
  
  # Kafka Brokers URLs.
  kafka_brokers = ["${var.my_kafka_broker_url}"]

  # Malka Configuration
  configuration = [
    # Each entry means a consumer group
    {
      # Topic to subscribe
      topic_name = "topic",

      # Number of parallel consumers for this topic.
      # It implies in the number of Lambda functions that will be invoked simultaneously.
      topic_number_of_consumers = 2,

      # Custom Kafka configuration for this consumer group.
      # For further details, please check the options available at the librdkafka documentation
      # https://github.com/edenhill/librdkafka/blob/master/CONFIGURATION.md
      consumer_configuration = {},
      
      # The AWS functions that will receive the notification
      target_functions = ["appetifyMailEngine"]
    }
  ]
}
```
For a complete example, please check the [example folder](https://github.com/miere/malka/blob/main/example/main.tf).


## License
Copyright 2021 - Malka maintainers and contributors

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
