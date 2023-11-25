# Project

The `Project` is our top-level, it contains everything.

| Name         | Required | Default | Type                | Description                            |
|--------------|----------|---------|---------------------|----------------------------------------|
| name         | yes      | -       | String              | Human readable name of the Project     |
| environments | no       | []      | List of Environment | The Environment for process execution  |
| containers   | no       | []      | List of Container   | Docker Containers that must be running |
| build        | no       | []      | List of BuildStep   | Steps to build a thing                 |
| services     | no       | []      | List of Service     | Services that must be running          |
| commands     | no       | []      | List of Command     | Commands that can be run               |

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
