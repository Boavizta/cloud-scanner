# AWS authentication

Easiest way to pass aws credential is use an environment variable to use a specific aws profile.

Pre-requisite: AWS CLI installed and configured: [Installing or updating the latest version of the AWS CLI - AWS Command Line Interface](https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html)

```sh
# cloud-scanner CLI uses the AWS_PROFILE set in the environment variable.
export AWS_PROFILE='<YOUR_PROFILE_NAME>'
```
