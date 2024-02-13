# Environment

The `Environment` is used to define environment variables.  
These variables can be used for templating and are passed to commands when they are executed.

| Name     | Required | Default | Type                    | Description                                       |
|----------|----------|---------|-------------------------|---------------------------------------------------|
| name     | yes      | -       | String                  | Human readable name of the Environment            |
| system   | no       | []      | List of String          | List of variables to copy from the system         |
| inherits | no       | []      | List of String          | List of other Environments to copy variables from |
| sources  | no       | []      | List of String          | List of .env files to copy from                   |
| values   | no       | {}      | Map of String to String | Map of string-value variables                     |

Environments have four sources of variables: system `system`, parent environments `inherits`, .env files `sources` and a key-value map `values`

These sources have a specific order in which they are loaded and overwritten.  
We call this order "precedence", and the last one in the order has the highest precedence.

The order in which the variables are loaded is the following:
1. `system`
   1. the given list of environment variables are loaded in the defined order
2. `inherits`
    1. the given list of environment names are loaded in the defined order
    2. the variables from the first environment are loaded first and are overwritten by the values of all following environments
    3. the variables from the last environment overwrite all prior-loaded variables, so it has the highest precedence.
3. `sources`
    1. the given list of `.env` files is loaded in the defined order
    2. all variables loaded from files overwrite variables that are copied from inherits
    3. the variables from the first file are loaded first and are overwritten by the values of all following files
    4. the variables from the last file overwrite all prior-loaded variables, so it has the highest precedence.
4. `values`
    1. values is a key-value map with string keys and string values
    2. any defined value overwrites variables that where copied from sources or inherits
    3. values always have the highest precedence

Example:
```Java
Project(
  name: "hello-world",
  environments: [
    Environment(
      name: "parent",
      system: ["HOME"],
      // sources are files, values from files are overwritten by values
      sources: ["hisho-parent.env"],
      // values, these override any variables from sources or inherits
      values: {
        "FOURTH": "4",
        "FIFTH": "5",
        "OVERRIDE_BY_VALUES": "yes",
      }
    ),
    Environment(
      name: "child",
      // inherit gets all values from the parents, last parent has the highest precedence
      inherits: ["parent"],
      // sources are files, values override parent values
      sources: ["hisho-child.env"],
      // values, again override any prior variables
      values: {
        "NINETH": "9",
        "TENTH": "10",
        "OVERRIDE_BY_CHILD_VALUES": "yes",
      },
    ),
  ],
)
```
