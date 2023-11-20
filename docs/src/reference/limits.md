# Limitations and perimeter

We do our best to offer accurate estimations of impacts.
But these figures should  still be considered with a grain of salt.

> All models are wrong, but some are useful.

The Boavizta impact data result from a modelling effort. It is important to consider the Perimeter of the resources that cloud-scanner accounts for and the limits of the tool itself.

## Perimeter / scope

Cloud scanner _only_ estimates the impacts of your AWS EC2  instances, and optionally block storage.

Several significant aspects of the global impacts of cloud usage are **excluded**:

- only estimate impacts of _compute_ instances (EC2 Virtual Machines).
- do not account for the surrounding cloud infrastructure (network, control plan).
- for **storage** (experimental feature), **only** the impacts of **manufacture** are counted. The impacts of the _use_ phase are _not_ counted. Boavizta API returns only the impacts of the _manufacturing_ phase for HDD and SSD. Furthermore the impacts of storage _are likely overestimated_. They are calculated by taking into account the size of the logical volume. In reality, a volume that is not full may not claim the full space on a physical device, which would result in smaller impacts.
- do not take into account the _over-commit_ (mutualization) or _over-provisioning_(redundancy) that cloud provider may apply to provide the service.
- do not account managed services (like DB as a service or Containers as a service).
- do not account serverless (lambda) compute.
- supported regions (EU and US only for the time being).
- unsupported instance types returns zero for their impacts.
- we do not provide (yet) error margins <https://github.com/Boavizta/boaviztapi/issues/147>.

## Other limits

### Not all Regions are supported

Only a subset of AWS regions are supported.
When performing inventory, he **only the EU-based or US-based** aws regions are supported for the time being (eu-east-1,,eu-central-1,eu-north-1,eu-south-1,eu-west-1,eu-west-2,eu-west-3,us-east-1,us-east-2,us-west-1,us-west-2).

When a region is not recognized by Cloud-scanner, it defaults to using the Carbon intensity factor of France, this is not a very good idea... In the future we should rather use a default average value or Europe or World instead.
This is particularly relevant when considering CoEq indicator because the carbon intensity factor varies greatly between regions.

### Instances supported

Cloud scanner supports all instances types of Boavizta API. But when a specific instance type cannot be matched with the Boavizta data set, Cloud-scanner returns empty impacts (i.e. zeroed impact values).

### Carbon intensity is not real time

Carbon intensity of electricity is not (yet) real time. It uses and yearly extract. This is more related to the API itself, see <https://github.com/Boavizta/boaviztapi/issues/117> and general Boavizta API methodology.

### We do not provide margins of error

Cloud scanner is not yet able to provide error margin related to it's estimation. Note that there is ongoing work on Boavizta API to provide more details about the error margins: <https://github.com/Boavizta/boaviztapi/issues/147>

### Open issues / bugs

See also [Issues · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues).
