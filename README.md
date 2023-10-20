# Hisho (秘書)

Hisho is a utility tool for local development with dependencies like Docker containers.

## Features

* Define Docker Containers that must be running, before any commands can be executed
* Define Build Steps that need to succeed, before a command is executed
* Configure environments for commands, these are separate from the system environment
* Human editable configuration format using RON (Rust Object Notation) for our `hisho.ron` files.

## Documentation

You can find the user documentation for Hisho in the `docs` directory.

The documentation is build using [JetBrains Writerside](https://www.jetbrains.com/writerside/).  
You may need to install Writerside into IntelliJ IDEA Ultimate or download standalone Writerside via the JetBrains Toolbox  
in order to write/build the documentation.  
Modification to pages can be done with any editor, as they are markdown or xml/html files.

## Supported Platforms

Hisho officially only supports Linux on AMD64 CPUs, being build with Rust 1.73+ and musl libc.

While Hisho tries to be platform agnostic, there is no guarantee that it works on other platforms.

## License

This code is licensed under the Mozilla Public License, 2.0.  
You can find a copy of the license in the LICENSE.txt file or [online](http://mozilla.org/MPL/2.0/).
