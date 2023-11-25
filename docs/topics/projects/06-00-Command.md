# Command

| Name             | Required | Default | Type                                | Description                                            |
|------------------|----------|---------|-------------------------------------|--------------------------------------------------------|
| name             | yes      | -       | String                              | Name of the build                                      |
| environment      | no       | []      | List of [Process](06-01-Process.md) | List of Environments to load for the process execution |
| shell            | no       | []      | List of String                      | List of Processes to execute in order                  |
| depends_on_build | no       | []      | List of String                      | List of build steps to run before running any shell    |


