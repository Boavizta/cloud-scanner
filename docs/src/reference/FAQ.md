# Frequently Asked Questions

## What are differences between Datavizta and Cloud scanner ?

Both tools rely on the same source of impact data (i.e. the [BoaviztAPI](https://api.boavizta.org/). The API is used to estimation the impacts of aws instances.

- [Datavizta](https://dataviz.boavizta.org/cloudimpact) is positioned as a _pedagogical_ front end to the API. It helps users understand which kind of data the API, quickly test hypothesis. It offers visualizations of impacts of multiple indicators for different lifecycle steps (use vs manufacture).

- Cloud scanner is more **production oriented**. It returns same data as Datavizta but _in different form and with different inputs_.
The main objective of cloud scanner is to help quickly retrieve the impacts of an entire AWS account by **automating the inventory** of resource used and the retrieval of associated impacts.

In addition cloud scanner **can be used as a monitoring system** that runs regularly. In this later case, it generates **results as metrics** (in standard _Prometheus / Open metrics_ format). Theses metrics can be consumed by external systems, to ease further analysis (e.g. breakdown by service / tags or countries) or use custom visualizations (like an external dashboard).

## Which cloud provider or service is supported ?

AWS (instances and block storage).

- Only AWS is supported for the time being.  The reason is that historically BoaviztAPI could only provide reference data for AWS instances.
- Azure instances are on the roadmap. After data becomes available in Boavizta API, cloud scanner will be adapted to support Azure https://github.com/Boavizta/cloud-scanner/issues/15.
- In addition BoaviztAPI is working on providing impacts of Storage as a service. It will be integrated to cloud scanner once available. https://github.com/Boavizta/boaviztapi/issues/143
