# Format of results in server mode

When run in server mode (`cloud-scanner-cli serve`), Cloud-scanner exposes three endpoints:

- `/metrics`: returns Prometheus metrics (plain text)
- `/inventory`: returns an inventory  (json format, see schema below)
- `/impacts`: returns impacts (json format, see schema below)

The most up-to-date OpenAPI specification is exposed under  `<BaseURL>/openapi.json` path and displayed using swagger-ui at `<BaseURL>/swagger-ui/index.html`.

## OAS spec

```json
{{#include openapi.json}}
```
