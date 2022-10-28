# CLI options

```sh

List aws instances and their environmental impact (from Boavizta API)

Usage: cloud-scanner-cli [OPTIONS] <COMMAND>

Commands:
  standard        Get Average (standard) impacts for a given usage duration (without considering cpu use)
  measured        Get impacts related to instances usage rate (take into account instance cpu  use)
  list-instances  Just list instances and their metadata (without impacts)
  help            Print this message or the help of the given subcommand(s)

Options:
  -a, --aws-region <AWS_REGION>
          AWS region (The default aws profile region is used if not provided)
  -b, --boavizta-api-url <BOAVIZTA_API_URL>
          Optional Boavizta API URL if you want to use your own instance (URL without the trailing slash, e.g. https://api.boavizta.org)
  -t, --filter-tags <FILTER_TAGS>
          Filter instances on tags (like tag-key-1=val_1 tag-key_2=val2)
  -v, --verbosity...
          Enable logging, use multiple `v`s to increase verbosity
  -m, --as-metrics
          Returns OpenMetrics (Prometheus) instead of json output
  -h, --help
          Print help information
  -V, --version
          Print version information
```
