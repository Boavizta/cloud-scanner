# CLI options

```sh
List aws instances and their environmental impact (from Boavizta API)

Usage: cloud-scanner-cli [OPTIONS] <COMMAND>

Commands:
  estimate   Get estimation of impacts for a given usage duration
  inventory  List instances and  their average cpu load for the last 5 minutes (no impact data)
  serve      Serve metrics on http://localhost:3000/metrics
  help       Print this message or the help of the given subcommand(s)

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
