# Process

| Name             | Required | Default | Type           | Description                                                      |
|------------------|----------|---------|----------------|------------------------------------------------------------------|
| command          | yes      | -       | String         | Path to the executable to run                                    |
| args             | no       | []      | List of String | List of arguments for the executable                             |
| cwd              | no       | -       | String         | Path where the command should be executed at (working directory) |

Example:
```Java
Project(
  name: "hello-world",
  build: [
    BuildStep(
      name: "list-tmp-files",
      shell: [
        Process(
          command: "ls",
          args:  [
            "-l",
          ],
          cwd: "/tmp"
        )
      ]
    )
  ],
)
```

## Arguments

Command Arguments can be Templated, see [Template Variables](Templating.md#variables_scopes).
