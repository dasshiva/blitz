# Blitz - A Virtual machine to ensure portability

Blitz is a project that aims to solve portability across platforms by providing a simple assembly language to target and a virtual machine which runs the assembled code. The same assembly runs on all platforms thus allowing code to be truly cross platform and WORA(Write-Once-Run-Anywhere), no matter which language it is written in.

Blitz is written in Rust and has two dependencies:
1. [mmap-rs](https://crates.io/crates/mmap-rs)
2. [file-utils](https://crates.io/crates/file-utils)

It has two components:
1. blc - The assembler for converting assembly code in Blitz assembly files (files with a .su extension) to machine code for the Virtual Machine (files with a .out extension)
2. blitz - The Virtual Machine to run the code 

Blitz is in its early stages of development but has the following features:
* A fully featured instruction set with 40 instructions including support for floating point arithmetic, conditional instructions and many more
* Support for functions and labels in assembly
* Memory protection to prevent random reads and writes to memory
* Exceptions (only system exceptions for the moment)
* 20 floating point(f0..f19) and 20 general purpose registers (r0..r19) each of size 64 bits
* Pushing and popping from the stack
* Single-line comments in assembly

You can find an example of how blitz assembly works by checking out the file [hello.su](hello.su) 

Blitz is in a very early stage of development and would really appreciate your suggestions on how to improve further.

To get an in-depth understanding as to how blitz works, check out the [specification](spec.md)

Copyright (C) 2023 Shivashish Das

Licensed under the [MIT License](LICENSE.md)
