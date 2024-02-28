# Limitations and perimeter

We do our best to offer accurate estimations of impacts.
But these figures should  still be considered with a grain of salt.

> All models are wrong, but some are useful.

The Boavizta impact data result from a modelling effort. It is important to consider:

1. the limited Perimeter of the resources that cloud-scanner takes into account.
2. the limits of estimations methodology.

## Perimeter / scope

Cloud scanner _only_ estimates the impacts of your AWS EC2  instances, and optionally block storage.

Several significant aspects of the global impacts of cloud usage are **excluded**:

- It only estimate impacts of _compute_ instances (EC2 Virtual Machines) and optionally block storage (HDD, SSD volumes).
- Cloud scanner does not take into account the PUE of the data center. See [Add an option to override PUE in the queries or cli option · Issue #422 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/422)
- Cloud scanner does not take into account the surrounding cloud infrastructure (network, control plan).
- For **storage** (experimental feature), **only** the impacts of **manufacture** are counted. The impacts of the _use_ phase are _not_ counted. At the moment, Boavizta API returns only the impacts of the _manufacturing_ phase for HDD and SSD. Furthermore the impacts of storage _are likely overestimated_. They are calculated by taking into account the size of the logical volume. In reality, a volume that is not full may not claim the full space on a physical device, which would result in smaller impacts.
- do not take into account the _over-commit_ (mutualization) or _over-provisioning_(redundancy) that cloud provider may apply to provide the service.
- do not account managed services (like DB as a service or Containers as a service).
- do not account serverless (lambda) compute.
- unsupported instance types returns zero for their impacts.
- Cloud scanner does not provide error margins <https://github.com/Boavizta/boaviztapi/issues/147>.

## Other limits

### Instances supported

Cloud scanner supports all instances types of Boavizta API. But when a specific instance type cannot be matched with the Boavizta data set, Cloud-scanner returns empty impacts (i.e. zeroed impact values).

### Carbon intensity of electricity is not real time

Carbon intensity of electricity is not real time. It uses and yearly average corresponding to the country where the data center is located. This is more related to the API itself, see <https://github.com/Boavizta/boaviztapi/issues/117> and general Boavizta API methodology.

### We do not provide margins of error

Cloud scanner is not yet able to provide error margin related to it's estimation. Note that there is ongoing work on Boavizta API to provide more details about the error margins: <https://github.com/Boavizta/boaviztapi/issues/147>

### Open issues / bugs

There may be additional identified issues or bugs, see [Issues](https://github.com/Boavizta/cloud-scanner/issues).
