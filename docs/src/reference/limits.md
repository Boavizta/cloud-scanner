# Limitations and perimeter

We do our best to offer accurate estimations of impacts.
But these figures should  still be considered with a grain of salt.

> All models are wrong, but some are useful.

The Boavizta impact data result from a modelling effort. It is important to consider the Perimeter of the reources that cloud-scanner accounts for and the limits of the tool itself.

## Perimeter / scope

Cloud scanner only provides impacts related to Compute resources (the Virtual Machines).

Several significant aspectof the cloud provider are **excluded**:

- only measure _compute_ instances (EC2 VM's)
- do not account for the surrounding cloud infrastructure (network, control plan)
- do not account for storage
- do not take into account the _overcommit_ (mutualization) or _overprovisionning_ that cloud provider may apply to provide the service.
- do not account managed services (like DB as a service or Containers as a service).
- do not account serverless (lambda) compute.
- supported regions (EU and US only for the time being)
- unsupported instance types returns zero for their impacts.
- we do not provide (yet) error margins https://github.com/Boavizta/boaviztapi/issues/147
- The manufacturing impacts are not amortized (i.e. values returned for manufacturing impacts do not consider usage duration).

## Other limits

### Not all Regions are supported

Only a subset of AWS regions are supported.
When performing inventory, he **only the EU-based or US-based** aws regions are supported for the time being (eu-east-1,,eu-central-1,eu-north-1,eu-south-1,eu-west-1,eu-west-2,eu-west-3,us-east-1,us-east-2,us-west-1,us-west-2).

When a region is not recognized by Cloud-scanner, it defaults to using the Carbon intensty factor of France.
This is particularly relevant when considering CoEq indicator because the carbon intensity factor varies greatly between regions.

### Instances supported

Cloud scanner supports all instances types of Boavizta API. But when a specific instance type cannot be matched with the Boavizta data set,
Cloud-scanner returns empty impacts (i.e. zeroed imapct values).

See also [Issues · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues).

### Carbon intensity is not real time

Carbon intensity of electricity is not (yet) real time. It uses and yearly extract. This is more related to the API itself, see <https://github.com/Boavizta/boaviztapi/issues/117> and general Boavizta API methodology.

### Allocation of manufacture impacts

Today, cloud scanner returns the manufacture impacts of a resource corresponding to *the entire lifecyle* of the ressource. The manufacture impacts returned for a VM are the same if you use it one hours or several year. Said differently we do *not* amortize the manufacuring impacts over the duration of use.

### We do not provide margins of error

Cloud scanner is not yet able to provide error margin related to it's estimation. Note that there is ongoing work on Boavizta API to provide more details about the error margins: <https://github.com/Boavizta/boaviztapi/issues/147>
