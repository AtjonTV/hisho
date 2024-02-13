# Service

The `Service` is a dependency.  
All defined Services must be reachable.

| Name     | Required | Default | Type                                        | Description                                 |
|----------|----------|---------|---------------------------------------------|---------------------------------------------|
| name     | yes      | -       | String                                      | Human readable name of the Service          |
| protocol | yes      | -       | [ServiceProtocol](05-01-ServiceProtocol.md) | Protocol of the service                     |
| uri      | yes      | -       | String                                      | Unified Resource Identifier for the service |

Hisho will exit if a service was not reachable, or the HTTP status was not 200 OK. 

Example:
```Java
Project(
  name: "hello-world",
  services: [
    Service(
      name: "cloudflare",
      uri: "https://cloudflare.com",
      protocol: HTTP
    )
  ],
)
```
