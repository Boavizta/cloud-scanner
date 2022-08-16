# Limitations

- Cloud-scanner return empty impacts when the instance _type_ is not listed in Boavizta database.
- `--aws-region` flag only supports eu-based aws regions for the time being (eu-east-1,eu-central-1,eu-north-1,eu-south-1,eu-west-1,eu-west-2,eu-west-3)
- Always returns _standard_ impacts: using instance workload to assess impact is not yet implemented (i.e. using CPU load through the `measured` command has no effect yet).
- Filtering instances by tag is not yet supported.
- Passing a private Boavizta API URL is not yet implemented
- The Boavizta Rust API SDK  (Boavizta API client for Rust) used by the cloud-scanner is maintained locally in the cloud-scanner repository, it may not reflect the latest changes of the API repository immediately.

See also [Issues · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues).
