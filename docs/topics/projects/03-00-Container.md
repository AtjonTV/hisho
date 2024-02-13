# Container

The `Container` is a dependency.  
All defined Containers must be running.  

| Name     | Required | Default | Type   | Description                        |
|----------|----------|---------|--------|------------------------------------|
| name     | yes      | -       | String | Docker Name or ID of the container |

All existing containers are fetched from the Docker Daemon.  
If the defined containers exist but are stopped, they will be started.

Hisho will exist if starting the defined containers failed, or if a container was not found.

Example:
```Java
Project(
  name: "hello-world",
  containers: [
    Container(
      name: "database"
    )
  ],
)
```
