# Testing

## Unit tests

Unit tests are launched with  `cargo test` command.

## End to end tests

When launched with `cargo test -- --include-ignored` additional integration tests need:

- specific instance to be running
- aws credentials to be setup

> These integration tests require specific instances to be up and running to pass. This means they are tied to a specific cloud account.

Memo: commands to start or stop instances:

```sh
# List instance state
aws ec2 describe-instance-status --include-all-instances --filters Name=instance-state-name,Values='*' --query 'InstanceStatuses[*].{InstanceId: InstanceId, State: InstanceState.Name}' --ouptut table

# start instance 
aws ec2 start-instances --instance-id i-03c8f84a6318a8186

# stop instance
aws ec2 stop-instances --instance-id i-03c8f84a6318a8186
```
