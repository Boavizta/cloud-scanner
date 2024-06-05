# Format of results in server mode

When run in server mode (`cloud-scanner-cli serve`), Cloud-scanner exposes 3 endpoints:

- `/metrics`: returns Prometheus metrics (plain text)
- `/inventory`: returns an inventory  (json format, see schema below)
- `/impacts`: returns impacts (json format, see schema below)

## Open API specification (Swagger)

To access the OpenAPI specification:

1. start a server (`cloud-scanner-cli serve`)
2. access the OpenAPI specification at <http://127.0.0.1:8000/openapi.json> and a swagger-ui at
<http://127.0.0.1:8000/swagger-ui/>


The latest (up-to-date) version of OpenAPI specification is exposed under  `<BaseURL>/openapi.json` path and displayed using swagger-ui at `<BaseURL>/swagger-ui/index.html`.

```json
{{#include openapi.json}}
```
