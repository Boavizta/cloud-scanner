# Using a private instance of Boavizta API

To avoid that the scanner sends your invnetory data to the public version of Boavizta API,  It is recommended to deploy your own instance of Boavizta API.

This is also usefull if you want to pin the version of API you intend to use.

The Boavizta API can be deployed in several ways (docker, lambda a.s.o.). Refer to the API documentation: [Deploy - Boavizta API documentation](https://doc.api.boavizta.org/Reference/deploy/).

Once you have deployed your instance of API, use either the command line flags (CLI) or environment variable (docker, lambda) to configure the scanner to use _your_ API instance.
