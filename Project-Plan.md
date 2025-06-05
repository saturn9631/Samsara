# Design
## Overview
- **Architecture:** As mentioned there will be 3/4 Core modules: Sometimes Architecture, Process and Communication, Memory and Security, and File and Resource. Each of these modules has one prime job and a secondary job related to their prime job. The Process Communication Server (PCS) handles process scheduling, and interprocess communication/networking. The PCS uses mailbox files and messages sent between them to build up all the other unix ipc (exempting semaphores): The connections, buffering, messages type of a mailbox can be configured, making them act function as other ipc or masquerade as another type of file (a mailbox connected to a image rendering process appearing as a jpeg). The PCS also handles the network stack down to the network layer (the bottom two are handled by the nic driver) since networking from the applications point of view is ipc and the mailbox system acts a lot like sockets. The Memory Security Server (MSS) is responsible for management of memory and controlling access to resources using permissions. Since most of memory management is securing memory, most security vulnerabilities are memory corruption, and resource access involve memory allocation/access, it makes since that the memory server should be responsible for general security. The File Resource Management (FRS) primary job is to manage the different file systems, with a second job of managing devices due to the Unix standard "everything is a file". The "everything is a file" standard is extended with the "every device is a drive": all hardware devices are treated like a storage device (which are also files) with a filesystem acting as an interface (i.g. the ports on a network card, or the framebuffer and processing clusters in a gpu represented as files in the devices filesystem). In addition, the FRS borrows from Plan 9 and Redox OS and makes every file path a url: the scheme is the filesystem, the host is the device, and the query at the end is Yggdrasil statement (see Yggdrasil Programming Language).
- **Yggdrasil Programming Language:** A table based language, Yggdrasil is going to be the programming language the kernel is programed in (after assembly bootstrapping) as well as the kernel being the primary compilier and interpreter of the language. The language will take primary inspiration from Rust, Lisp, Lua, and assembly with some other features taken from C++, Zig, Java. The central complex data structure is the table, with structs being the rows and enums being the columns. Types can also be treated as values and manipulated allowing for metaprogramming, the definition of operators and the extension of the language. Permissions, a merging of Unix file permissions and Rust's mutability, allows for control of data access. Permissions are usually managed as part of an object and are given to references and slices. The point is that operations dependend on the memory they are operated on: allocation is not only used to get heap memory, but is how files are accessed and threads are created and accessed.

## Ideas
- **Architecture:**
    - There might be a distinction between servers and drivers. As of current there is no distinction.
- **Yggdrasil:**
    - Structs and enums are algebraic data types, with structs being product types and enums being sum types
    - Structs are tuples. All arrays are tuples with numeric index.
    - Enums are tagged unions.
    - Permissions are attached to a resource that states what can be done to it: read, write, or execute, and these permission are the same as in files and memory pages/segments. Lifetimes are also a type of permission.
    - File inodes are structs.




# Design and Goals
# Implementing
## Features
- **Architecture:**
- **Language:**
    - **Objects are Collections:** Ever object is a collection and every collection is an object. This means that objects can be indexed and collections have methods.
    - **Type Hierachy:** types can be terms of other types. This is how polymorphism and meta programming is handled: a variable of a supertype will hold any of its subtypes and modifying the supertype to a values type will also modify the behavior of that value.
    - **References and Slices:** Like in rust references point to a value and thus "barrow" it, with slices being a multi-entity reference both point to a specific entity and barrow that entity. The difference lies in the fact that barrowing requires permission (see "permissions" in "Memeory Security Manager") that tells what can be barrowed and what can be done to it.
    - **Operators/Iterators:** These are in way one in the same (see "Objects and collections are equivalent") since operators act on objects (such as in c++ and python) and iterators act on lists. Both operators and iterators are immutable by default and us copy-on-write, although mutating operators can be made, they would require that said operands have the correct permissions attached.
- **Core Modules:**
    - **Process Scheduling Manager (PSM):** Schedules processes and handles interprocess communication (ipc)
        - **Mailboxes and Message:** Files that can be written to to send a message and read from to recieve a message. Many characteristics of a mailbox can be changed such as the queing of messages, the destinations and sources of the messages, and the type of messages. All other unix based ipc will be specially configured Mailboxes. Networking is done by formatting the messages as packets/frames and then passing them through the different layers.
        - **Supervisor Tree (Kernel Mode):** Taking inspiration from Erlang, all kernel mode processes and threads must have a supervisor that is its parent and handles its returns. Threads are spawned to execute a task and do not handle their errors, instead opting to return it to their supervisor, either a process or another thread, that then decides what to do with it, e. g. ignore it, reattempt, or close itself and notify its supervisor. Processes are the execution state of a module whose main job is to supervise threads. A process can only have another process as its supervisor, although a thread can still request the start of a process from its supervisor, with the PSM being the top level supervisor, with errors escaping it causing a kernel panic.
        - **Stacks and Functions (Language Feature):** Every thread is really just a call stack, with each function/closure correspounding to a frame. A new thread is created by placing a function on the heap by attaching the "new" keyword (used for creating heap allocated objects) while calling the function, at which point the function becomes the top level of the thread (for threads in kernel space this function has to return something). Every block that is not a function is a closure and many of the of the familar termination keywords: break, continue, and return, are used to manipulate the blocks on the stack.
    - **Memory Security Manager (MSM):** Manages memory and controls access to it and other resources.
        - **Users:** A collection of ownership and priviledges. Users can also be parents to other users, which is how user groups are implemented. Many other entities can also be users: processes, devices, , etc.
        - **Permissions:** The same as unix permissions, the only difference is that the group octal is now the parent octal, denoting what a parent can do with the resource (a child can usually do what the parent can do).
        - **Modifiers (Language Feature):** Determines what permissions a caller has. Modidiers not only encode the read, write and execute (call) permissions, which result in immutable, mutable, and function references respectively, but also whether it is visible and can be accessed.
    - **File Resource Manager (FRM):** Keeps track of files, devices/drives, and other resources.
        - Drives: All devices, real or virtual, are treated as storage drives in an extension of Unix's "Everything's a file".
        - **URL Paths:** All file paths are urls: the scheme is the filesystem, the host is the device (drive) that the resource is on, the path is the same as the filepath, the query being a Yggdrasil expression to be evaluated by the kernel.
        - **Streams and (Language Feature):** All files (more specifically their inodes) are types with the directory tree being its type structure: a files type is its directory
## Build Plan:
- Bootloader:
    - List stuff on the drive.
    - Find and load kernel models.
    - Set runtime variable to allow binder to run after boot
- Kernel:
    - Implement core modules and binder.
- Compiler:
    - Paser that breaks the source code into blocks, the blocks into statements, and statements into token.
    - Pair that parser to the llvm.
# Planned
- **File Systems:**
    - **Tagging File System (TFS):**
    - **Database File System (DBFS):**
    - **Linear Blockchain Access File System (LBAFS) and Archival Block Chain File System (ABCFS):**
- **Heterogenous Computing:**
    - **Alderlake E and P cores:**
    - **ESP32 Sleep mode execution:**
    - **Tensor and Raytrace core control:**
- **Architecture Compatabillity:**
    - Arm
    - Coreboot
    - RISC V
    - x86
# Considering
- Modular numbers:
- Algebriac Data Types:
# Tools and Knowledge
The llvm assembler was ultimately chosen due to its toolchain customizability and its compatibility. Since Samsara and Yggdrasil have planned Rust interroptability and Cuda translation, it makes since that Yggdrasil would be translated into the same IR as the both of them. LLVM also has backends for x86, Arm, Risc-V and WebAssembly, allowing for Samsara/Yggdrasil to be available on these platforms. This decision is subject to reassessment and can change if another toolchain is deemed superior.

## Toolchain Assement
- **LLVM:**
    Pros:
        - Toolchain customization
        - Wide hardware compatibility
        - Wide language interropability
        - Prebuilt tools (C++ standard library, LLDB debugger, JIT, etc.)
    Cons:
        - Longer build times
- **GCC:**
    Pros:
        - Fast build time
    Cons:
        - Toolchain inflexibility
        - GPL (in the case that MIT License is desired)
- **NASM:**
    Pros:
        - More toolchain control and customization
    Cons:
        - Labor associated with creating/integrating tools into toolchain
