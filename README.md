# Hisho (秘書)

Hisho is a utility tool for local development with dependencies like Docker containers.

## Features

* Define Docker Containers that must be running, before any commands can be executed
* Define Build Steps that need to succeed, before a command is executed
* Configure environments for commands, these are separate from the system environment
* Human editable configuration format using RON (Rust Object Notation) for our `hisho.ron` files.

## Supported Platforms

Hisho officially only supports Linux on AMD64 CPUs, being build with Rust 1.73+ and musl libc.

While Hisho tries to be platform agnostic, there is no guarantee that it works on other platforms.

## License

This code is licensed under the Mozilla Public License, 2.0.  
You can find a copy of the license in the LICENSE.txt file.
