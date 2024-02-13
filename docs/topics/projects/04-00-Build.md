# Build

The `Build` is a dependency.  
All defined Builds must be succeeding.

| Name        | Required | Default | Type                                | Description                                           |
|-------------|----------|---------|-------------------------------------|-------------------------------------------------------|
| name        | yes      | -       | String                              | Name of the build                                     |
| shell       | no       | []      | List of [Process](06-01-Process.md) | List of Process to execute                            |
| depends_on  | no       | []      | List of String                      | List of build-steps to depend on                      |
| input_files | no       | []      | List of String                      | List of globs to collect file paths for `input_files` |

Example:
```Java
Project(
  name: "hello-world",
  build: [
    BuildStep(
      name: "release",
      shell: [
        Process(
          command: "cargo",
          args:  [
            "build", "--release"
          ]
        )
      ]
    ),
    BuildStep(
      name: "compile",
      shell: [
        Process(
          command: "rustc",
          args:  [
            "-o", "a.out", "[[argv]]"
          ]
        )
      ],
      input_fiels: ["src/*.rs"]
    )
  ],
)
```


## Process

See [Command Process](06-01-Process.md) for details.
