# Get impacts of an arbitrary inventory

This is useful to simulate the impacts of a non-existing infrastructure or a variant of existing infrastructure.

This involves building an inventory file and passing it to cloud scanner for evaluation.

1. describe an arbitrary inventory (the to-be infrastructure) as a json file.
2. request cloud scanner to estimate impacts of this inventory. 

💡 It may be easier to adapt an existing inventory file, rather than creating it from scratch. See [Estimate the impacts of an existing inventory](estimate-from-existing-inventory-file.md).

The JSON schema of the inventory file is in the git repository: [cloud-scanner/cloud-scanner-cli/test-data/INVENTORY_JSON_SCHEMA.json](https://github.com/Boavizta/cloud-scanner/blob/main/cloud-scanner-cli/test-data/INVENTORY_JSON_SCHEMA.json). This schema can also be retrieved with the command `cargo run inventory --print-json-schema`.
