# Templating

One of the features of Hisho is that certain parts of the configuration can be templated.

The topics for build steps and commands already scratch the surface of the templating functionality.  
But they do not cover what the templating can actually do.

We use the rust implementation of the [Shopify Liquid](https://shopify.github.io/liquid/) templating engine.  
To allow you the most customizable templating, we enabled all the standard filters.

## Variable scopes

We use three different scopes for different types of variables to be used in templates.  
With scope, we actually mean a object that has the variables as properties on it.

Some scopes are always accessible and some are relative to the context in which the template is used.  
For example, the `arg` scope is only available within the Process object of an Command.

| Scope | Available in                              | Description                                                                                                                                   |
|-------|-------------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------|
| `arg` | Processes inside Commands                 | When you define `{{ arg.my_value }}` as a template and run Hisho with `--my_value=the-value`, the result of the template will be `the-value`. |
| `env` | Processes inside Build Steps and Commands | For Build Steps the available variables come from the command that depends on the step. For Commands it comes from the defined `environment`  |
| `git` | Processes inside Build Steps and Commands | Always available but only filled when the configuration file is inside of a git repository                                                    |

### The Arg scope

The available variables inside the `arg` scope depend on the command line input.

### The Env scope

The available variables inside the `env` scope depend on the environment selected for the Command.

### The Git scope

The available variables inside the `git` scope are always the same as defined below.  
These variables are empty by default and only filled when the configuration file is inside of a git repository with at least one commit.

| Name | Description                                       |
|------|---------------------------------------------------|
| `commit_sha` | The full SHA1 hash of the latest commit      |
| `commit_sha_short` | The short SHA1 hash of the latest commit     |
| `commit_date` | The ISO8601 formatted date of the latest commit |
| `commit_author_name` | The name of the author of the latest commit  |
| `commit_author_email` | The email of the author of the latest commit |

## Important note about compatibility

The rust implementation of Liquid ([liquid-rust](https://github.com/cobalt-org/liquid-rust)) and our use of it, does not allow for default values to be used.  
If a variable does not exist, Hisho will exit and complain about the missing variable.

While the Liquid documentation states that `{{foo | default: 'meow'}}` should result in the string "meow" to be the result of the template when foo is undefined, this is not what will happen.

This behaviour is similar to the `strict_variables` feature of the Shopify Liquid Ruby gem.

There is an [open issue on liquid-rust](https://github.com/cobalt-org/liquid-rust/issues/477) about this topic.
