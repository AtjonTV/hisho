# Project

The `Project` is our top-level, it contains everything.

| Name         | Required | Default | Description                        |
|--------------|----------|---------|------------------------------------|
| name         | yes      | -       | Human readable name of the Project |
| environments | no       | []      | List of Environment Objects        |
| containers   | no       | []      | List of Container Objects          |
| build        | no       | []      | List of BuildStep Objects          |
| commands     | no       | []      | List of Command Objects            |

Example:
```Java
Project(
  name: "hello-world",
  environments: [],
  containers: [],
  build: [],
  commands: [], 
)
```
