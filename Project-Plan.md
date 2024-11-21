# Stages:
1. **Boot Test Stage:** Make a simple kernel that boots into qemu (or other virtual machine)
    1. Implement Multiboot2 Standard
2. **Protocols Stage:** Start making modules.
3. **Expand:**

# Design
- **Architecture:** As mentioned there will be 3/4 Core modules: Sometimes Architecture, Process Scheduling, Memory Security, and File Resource. Each of these modules has one prime job and a secondary job related to their prime job. The Process Scheduling Manager (PSM) handles process scheduling, and interprocess communication/networking. The PSM uses mailboxes and 
- **Yggdrasil Programming Language:** A table based language, Yggdrasil is going to be the programming language the kernel is programed in (after assembly bootstrapping) as well as the kernel being the primary compilier and interpreter of the language. The language will take primary inspiration from Rust, Lisp, Lua, and assembly with some other features taken from C++, Zig, Java. The central complex data structure is the table, with structs being the rows and enums being the columns. Types can also be treated as values and manipulated allowing for metaprogramming, the definition of operators and the extension of the language. Permissions, a merging of Unix file permissions and Rust's mutability, allows for control of data access. Permissions are usually managed as part of an object and are given to references and slices. The point is that operations dependend on the memory they are operated on: allocation is not only used to get heap memory, but is how files are accessed and threads are created and accessed.

# Planned Features
- **File Systems:**
    - **Cyan File System (CFS):**
    - **Database File System (DBFS):**
    - **Archival Block Chain FIle System (ABCFS):**
- **Arm Sleep mode execution:**
- **Self-Booting:**
