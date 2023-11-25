# Process

| Name             | Required | Default | Type           | Description                                                      |
|------------------|----------|---------|----------------|------------------------------------------------------------------|
| command          | yes      | -       | String         | Path to the executable to run                                    |
| args             | no       | []      | List of String | List of arguments for the executable                             |
| cwd              | no       | -       | String         | Path where the command should be executed at (working directory) |

## Arguments

Command Arguments can be Templated, see [Template Variables](Templating.md#variables_scopes).