# Get impacts of an existing inventory file

You can use cloud scanner to take a snapshot of an existing inventory and run estimation of impacts later.

## Get an inventory file

### From command line

```shell
# Save an inventory to file
cloud-scanner inventory > my_inventory.json
```

### Using http server / API mode

```shell
# start server
cloud-scanner serve
# get the inventory
curl -X 'GET' \
  'http://localhost:8000/inventory?aws_region=eu-west-3&include_block_storage=true' \
  -H 'accept: application/json' > my_inventory.json
```


## Get estimate for a existing inventory

### From command line

_You have to explicitly pass the duration of use for which you would like to retrieve the estimation._

```sh
# Get impacts of an existing inventory 
cloud-scanner estimate --use-duration-hours 1 --inventory-file my_inventory.json
```

### Using http server / API mode

_You have to explicitly pass the duration of use for which you would like to retrieve the estimation._

```shell
# start server
cloud-scanner serve
# Post the previously saved inventory (my_inventory.json) for estimate
curl -X POST -H "Content-Type: application/json" -d @my_inventory.json http://localhost:8000/impacts-from-arbitrary-inventory\?use_duration_hours\=10\&verbose_output\=false
```
