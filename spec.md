# The Blitz Specification 
Copyright (C) 2023 Shivashish Das

This specification describes Blitz, a hardware architecture implementated in software and its associated details.

# Registers
Blitz has 20 general purpose registers (r0 to r19) and 20 floating point registers, both of which are 8 bytes in size.
The general purpose registers can be accessed as smaller sizes as well:
* b0 to b19 - accesses the lower 8 bits of the corresponding 8 byte register
* w0 to w19 - accesses the lower 16 bits of the same
* d0 to d19 - accesses the lower 32 bits of the same

Note that using the registers this way will clear the upper bits i.e if you store into d0, the upper 32 bits of r0 get cleared to 0.

# Instruction format
Blitz instructions are variable length instructions containing 1 descriptive prologue (4 bytes) followed by optional immediate arguments (each of 8 bytes and maximum 3 in number)

## Prologue
The prologue describes the instruction and its format. It is divided into 3 parts:
1. Opcode - The upper 10 bits of the prologue represent the opcode
2. Clusters - The next 21 bits are divided into 3 clusters of 7 bits each. Each cluster can have values in the following ranges which represent the following:
* 0 to 19 - Represent the 1 byte registers (b0 to b19) or the 8 byte floating point registers (f0 to f19) depending upon the specific instruction (i.e for mov it represents b0 to b19 while for fmov it represents f0 to f19)
* 20 to 39 - Represent the 2 byte registers (w0 to w19)
* 40 to 59 - Represent the 4 byte registers (d0 to d19)
* 60 to 79 - Represent the 8 byte registers (r0 to r19)
* 80 - Represent the stack pointer 
* 81 to 83 - Represent special meanings in the immediates as explained later (
3. Privilege - The last bit is 1 if this is a privileged opcode and 0 if not. An instruction is privileged if it is a part of a function having a firmware attribute

(To see the code encoding the instructions have a look at the functions sem_analyse in sema.rs and Args::new() in parse.rs)

## Immediates
If any of the clusters have the values between 81 to 83 then the prologue is followed by 8 byte immediate arguments in the following manner:
* 81 - Followed by a 8 byte signed integer
* 82 - Followed by a 8 byte floating point number
* 83 - Followed by a offset from a register. An offset means the value stored in the register plus some value. They are used to address memory. The upper 7 bits represent the register and registers are encoded in the same way as in the clusters in the prologue. The lower 57 bits represent the offset from the value in the register. Offsets are used for representing notations like [r0 + 1].

# Memory
Blitz can only use 2 MB of system memory currently though it is planned to remove this restriction in the near future. Memory is divided into four segments:
1. Code - Stores all code loaded from input file
2. Data - Stores static data from the user loaded file
3. Stack - Behaves as the stack memory
4. Heap - Behaves as heap memory

Blitz has a tight memory protection system. Only the code segment is marked read, write, execute i.e the processor will refuse to execute xode from any other segment other than code. This also means the code segment can be written and read from allowing code to be dynamically generated at runtime if so needed.
Data segment is marked read-only making it ideal for constants and the like.
Both the stack and heap segments are marked read-write for their normal operations.

# Calling convention
* Registers r0 to r6 are used for passing paramaters between functionncalls
* Registers r7 to r14 are to be saved across function calls
* Registers r14 to r19 are scratch registers 
* Stack must be 8-byte aligned at all times
* Stack grows downwards.
