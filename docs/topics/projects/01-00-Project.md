# Project

The `Project` is our top-level, it contains everything.

| Name         | Required | Default | Type                                        | Description                            |
|--------------|----------|---------|---------------------------------------------|----------------------------------------|
| name         | yes      | -       | String                                      | Human readable name of the Project     |
| environments | no       | []      | List of [Environment](02-00-Environment.md) | The Environment for process execution  |
| containers   | no       | []      | List of [Container](03-00-Container.md)     | Docker Containers that must be running |
| build        | no       | []      | List of [BuildStep](04-00-Build.md)         | Steps to build a thing                 |
| services     | no       | []      | List of [Service](05-00-Service.md)         | Services that must be running          |
| commands     | no       | []      | List of [Command](06-00-Command.md)         | Commands that can be run               |

Example:
```Java
Project(
  name: "hello-world",
  environments: [],
  containers: [],
  build: [],
  services: [],
  commands: [], 
)
```
