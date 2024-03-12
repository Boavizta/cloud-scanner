# Using a private instance of Boavizta API

To avoid that the scanner shares your inventory data to the public version of Boavizta API,  It is recommended to deploy your own instance of Boavizta API.

This is also useful if you want to stick to a specific version of the API.

The Boavizta API can be deployed in several ways (docker, lambda a.s.o.). Refer to the API documentation: [Deploy - Boavizta API documentation](https://doc.api.boavizta.org/Reference/deploy/) or refer to the provided [docker compose example](../tutorials/quickstart-dashboard-docker.md) for an example of integration cloud-scanner and a private instance of the API.

Once you have deployed your instance of API, use either the command line flags (CLI) or environment variable (docker, lambda) to configure the scanner to use _your_ API instance.
