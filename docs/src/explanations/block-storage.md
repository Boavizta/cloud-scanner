# Estimating impacts of block storage

Block storage volumes are approximated either as SSD or HDD depending on cloud provider specific storage type.

This is a basic estimation. It takes into account the size of the block storage volume.

⚠ Only the **manufacture** impacts of the Block storage are returned. The impacts of the **use** phase are returned as zero (mainly because we lack the related data in current version of Boavizta API).

## Block storage (EBS) estimation (AWS)

### What is considered in estimation:

- storage type (HDD vs SSD)
- storage size
- duration of use

| AWS EBS volume type | Boavizta storage used for estimation | comments |
| :------------------ | :----------------------------------- | :------- |
| st1                 | HDD                                  |          |
| sc1                 | HDD                                  |          |
| magnetic (standard) | HDD                                  |          |
| gp2                 | SSD                                  |          |
| gp3                 | SSD                                  |          |
| io1                 | SSD                                  |          |
| io2                 | SSD                                  |          |
| unknown type        | SSD                                  | If it cannot identify the type of storage (like when a new type is introduced, cloud scanner uses SSD as an approximation)         |

### What is not considered for the estimation

- If the storage offers provisioned IOPS or not.
- Implicit redundancy related to storage type.
- Any other infrastructure (network, backup, supervision or redundancy) that may allow the cloud provider to offer EBS service.

## Block storage estimations (Azure)

Not yet implemented.
