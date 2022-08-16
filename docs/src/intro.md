# About cloud-scanner

🌳 This application returns environmental impacts of your AWS Cloud usage. It combines Boavizta data and metadata of your AWS cloud account to offer a global view of your impacts on a given region.

Use it as a command line _or_ serverless application. 

Cloud-scanner analyses your EC2 instances and returns metrics using the [Boavizta API](https://github.com/Boavizta/boaviztapi/).

⚠ Work in progress ! See the [changelog](https://github.com/Boavizta/cloud-scanner/blob/main/CHANGELOG.md).

At the moment it only returns _standard_ impacts of AWS instances. It does not yet analyses instance usage (workload) to calculate the impacts, but rather returns the _default_ impact data provided by Boavizta API for each instance type for a fixed duration of use.

![Scanner in context](../../out/docs/cloud-scanner-system-in-context/cloud-scanner-system-in-context.png)