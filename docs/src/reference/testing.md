# Testing

When launched with `cargo test -- --include-ignored` some the unit tests require a specific instance to run (when launched ).

> These integration tests require specific instances to be up and running to pass. This means they are tied to a specific cloud account.

Commands to start or stop instances:

```sh
# List instance state
aws ec2 describe-instance-status --include-all-instances --filters Name=instance-state-name,Values='*' --query 'InstanceStatuses[*].{InstanceId: InstanceId, State: InstanceState.Name}' --ouptut table

# start instance 
aws ec2 start-instances --instance-id i-03c8f84a6318a8186

# stop instance
aws ec2 stop-instances --instance-id i-03c8f84a6318a8186

``` 