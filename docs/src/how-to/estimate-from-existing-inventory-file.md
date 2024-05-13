# Get impacts of an existing inventory file

You can use cloud scanner to take a snapshot of an existing inventory and run estimation of impacts later.

## Get an inventory file

```shell
# Save an inventory to file
cloud-scanner inventory > my_inventory.json
```

## Get estimate for a existing inventory

```sh
# Get impacts of an existing inventory 
cloud-scanner estimate --use-duration-hours 1 --inventory-file my_inventory.json
```

