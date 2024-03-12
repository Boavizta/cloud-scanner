# Methodology and source of data

Cloud scanner uses the Boavizta methodology to estimate the impacts of cloud resources.

## Source of impact data

Impact data is retrieved from [BOAVIZTA reference data API](https://github.com/Boavizta/boaviztapi/) v1.1.x.

## Methodology

The general approach of Boavizta is described in [Digital &amp; environment : How to evaluate server manufacturing footprint, beyond greenhouse gas emissions? | Boavizta](https://boavizta.org/en/blog/empreinte-de-la-fabrication-d-un-serveur)

The impacts (use and embedded) are attributed according to the principles described in [Cloud instances - Boavizta API documentation](https://doc.api.boavizta.org/Explanations/devices/cloud/).

The results are similar to what you can visualize in [Datavizta](http://datavizta.boavizta.org/cloudimpact), but with automated inventory.

⚠ Cloud scanner **underestimate the impacts of the cloud resources**. Because it only considers the _instances_ and _block storage_ a lot of impacts (network, potential redundancy, cloud control plan) are not included in the estimation.

See also [other limits](../reference/limits.md).

- https://www.boavizta.org/en
