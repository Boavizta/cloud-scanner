# Limitations

- Only EU region are supported: `--aws-region` flag only supports eu-based aws regions for the time being (eu-east-1,eu-central-1,eu-north-1,eu-south-1,eu-west-1,eu-west-2,eu-west-3)
- Cloud-scanner return empty impacts if the instance _type_ is not listed in Boavizta database.
- Filtering instances by tag is not yet supported.

See also [Issues · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues).
