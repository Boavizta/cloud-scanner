```sh
curl -X 'POST' \
  'http://api.boavizta.org:5000/v1/cloud/aws?instance_type=a1.4xlarge&verbose=true' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{
  "hours_use_time": 2,
  "usage_location": "FRA",
  "workload": {
    "10": {
      "time": 0
    },
    "50": {
      "time": 1
    },
    "100": {
      "time": 0
    },
    "idle": {
      "time": 0
    }
  }
}'
```


```sh
curl -X 'POST' \
  'http://api.boavizta.org:5000/v1/cloud/aws?instance_type=a1.4xlarge&verbose=false' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{ "usage" :{
  "hours_use_time": 4,
  "usage_location": "FRA",
  "workload": {
    "10": {
      "time": 0
    },
    "50": {
      "time": 1
    },
    "100": {
      "time": 0
    },
    "idle": {
      "time": 0
    }
  }}
}'
{"gwp":{"manufacture":565,"use":100.8786708},"pe":{"manufacture":7720.0,"use":"Not Implemented"},"adp":{"manufacture":0.102,"use":"Not Implemented"}}%
```


```sh
curl -X 'POST' \
  'http://api.boavizta.org:5000/v1/cloud/aws?instance_type=a1.4xlarge&verbose=false' \
  -H 'accept: application/json' \
  -H 'Content-Type: application/json' \
  -d '{ "usage" :{}}'
```
