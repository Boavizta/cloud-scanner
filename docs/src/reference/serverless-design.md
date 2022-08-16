# Serverless design

We use the [serverless framework](https://www.serverless.com/) and the [softprops/serverless-rust](https://github.com/softprops/serverless-rust) plugin to ease packaging and deployment as lambda.

The cloud-scanner-cli is wrapped into a set of lambdas functions exposed behind an AWS API gateway.

_This is certainly not the only way to deploy the application. If you want more control, you could compile, package and deploy the application with Terraform or CDK, but this is not documented yet._
