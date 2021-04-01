# malka
A service that leverages AWS Lambda as first-class Kafka consumers.

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

## Documentation
`TBD`

## Reporting Bugs/Feature Requests
We welcome you to use the GitHub issue tracker to report bugs or suggest features.

When filing an issue, please check existing open, or recently closed, issues to make sure somebody else hasn't already
reported the issue. Please try to include as much information as you can. Details like these are incredibly useful:

* A reproducible test case or series of steps
* The version of our code being used
* Any modifications you've made relevant to the bug
* Anything unusual about your environment or deployment


## Contributing via Pull Requests
Contributions via pull requests are much appreciated. Before sending us a pull request, please ensure that:

1. You are working against the latest source on the *master* branch.
2. You check existing open, and recently merged, pull requests to make sure someone else hasn't addressed the problem already.
3. You open an issue to discuss any significant work - we would hate for your time to be wasted.

To send us a pull request, please:

1. Fork the repository.
2. Modify the source; please focus on the specific change you are contributing. If you also reformat all the code, it will be hard for us to focus on your change.
3. Ensure local tests pass.
4. Commit to your fork using clear commit messages.
5. Send us a pull request, answering any default questions in the pull request interface.
6. Pay attention to any automated CI failures reported in the pull request, and stay involved in the conversation.

GitHub provides additional document on [forking a repository](https://help.github.com/articles/fork-a-repo/) and
[creating a pull request](https://help.github.com/articles/creating-a-pull-request/).


## Finding contributions to work on
Looking at the existing issues is a great way to find something to contribute on. As our projects, by default, use the default GitHub issue labels ((enhancement/bug/duplicate/help wanted/invalid/question/wontfix), looking at any 'help wanted' issues is a great place to start.

## License
This is release under the Apache License 2 terms.
