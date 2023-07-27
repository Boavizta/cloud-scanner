# Format of results in server mode

When run in standalond server mode (`cargo run serve`), Cloud-scanner exposes three endpoints:

- `/metrics`: returns Prometheus metrics (plain text)
- `/inventory`: returns an inventory  (json format, see schema below)
- `/impacts`: returns impacts (json format, see schema below)

The most up-to-date OpenAPI specification is exposed under  `/openapi.json` path.

## OAS spec

```json
{{#include openapi.json}}
```
