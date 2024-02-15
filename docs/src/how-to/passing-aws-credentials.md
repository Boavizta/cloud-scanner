# AWS authentication

Cloud scanner uses AWS Rust SDK to query AWS account. By default Rust SDK picks up AWS credentials from environment variables.

## AWS permissions required by Cloud Scanner

🔥 An Important pre-requisite is to have a *user* with sufficient permissions to list resources of the account (or an instance profile / role pre-configured when running cloud-scanner from EC2).

The minimal set of permissions to perform inventory of resources (and query CPU load of instances) is:

- ec2:DescribeInstances
- cloudwatch:GetMetricStatistics
- cloudwatch:DescribeAlarm

You could also restricts permissions to a specific set of instances or resources.

## Pass credentials as environment variables

### Option 1: Use AWS_ACCESS_KEY_ID and AWS_SECRET_ACCESS_KEY

Set environment variables with your account detail.

```sh
# Example for Linux / macOS
export AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE
export AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
export AWS_DEFAULT_REGION=eu-west-1
```

*See <https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-envvars.html> for equivalent Windows command prompt or Powershell syntax example.*

### Option 2:  Use a an existing AWS_PROFILE

If you have configured an AWS CLI profile, the easiest way to pass aws credential to cloud scanner  is use an environment variable that points to this profile.

Pre-requisite to use a profile:

1. AWS CLI *installed*: [Installing or updating the latest version of the AWS CLI - AWS Command Line Interface](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html) 
2. AWS CLI *configured*: [Configure the AWS CLI - AWS Command Line Interface](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-configure.html).

```sh
# Example for Linux / macOS
# cloud-scanner can use the AWS_PROFILE set as en environment variable.
# You have to reference one of the profiles previously configured in ~/.aws/credentials
export AWS_PROFILE=YOUR_PROFILE_NAME_EXAMPLE
export AWS_DEFAULT_REGION=eu-west-3
```

*See <https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-envvars.html> for equivalent Windows command prompt or Powershell syntax example.*

## Reference

See [Specify your credentials and default Region - AWS SDK for Rust](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/credentials.html) for more options to pass credentials to the SDK.
