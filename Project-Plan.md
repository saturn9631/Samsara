# Design
- **Architecture:** As mentioned there will be 3/4 Core modules: Sometimes Architecture, Process and Communication, Memory and Security, and File and Resource. Each of these modules has one prime job and a secondary job related to their prime job. The Process Communication Server (PCS) handles process scheduling, and interprocess communication/networking. The PCS uses mailbox files and messages sent between them to build up all the other unix ipc (exempting semaphores): The connections, buffering, messages type of a mailbox can be configured, making them act function as other ipc or masquerade as another type of file (a mailbox connected to a image rendering process appearing as a jpeg). The PCS also handles the network stack down to the network layer (the bottom two are handled by the nic driver) since networking from the applications point of view is ipc and the mailbox system acts a lot like sockets. The Memory Security Server (MSS) is responsible for management of memory and controlling access to resources using permissions. Since most of memory management is securing memory, most security vulnerabilities are memory corruption, and resource access involve memory allocation/access, it makes since that the memory server should be responsible for general security. The File Resource Management (FRS) primary job is to manage the different file systems, with a second job of managing devices due to the Unix standard "everything is a file". The "everything is a file" standard is extended with the "every device is a drive": all hardware devices are treated like a storage device (which are also files) with a filesystem acting as an interface (i.g. the ports on a network card, or the framebuffer and processing clusters in a gpu represented as files in the devices filesystem). In addition, the FRS borrows from Plan 9 and Redox OS and makes every file path a url: the scheme is the filesystem, the host is the device, and the query at the end is Yggdrasil statement (see Yggdrasil Programming Language).
- **Yggdrasil Programming Language:** A table based language, Yggdrasil is going to be the programming language the kernel is programed in (after assembly bootstrapping) as well as the kernel being the primary compilier and interpreter of the language. The language will take primary inspiration from Rust, Lisp, Lua, and assembly with some other features taken from C++, Zig, Java. The central complex data structure is the table, with structs being the rows and enums being the columns. Types can also be treated as values and manipulated allowing for metaprogramming, the definition of operators and the extension of the language. Permissions, a merging of Unix file permissions and Rust's mutability, allows for control of data access. Permissions are usually managed as part of an object and are given to references and slices. The point is that operations dependend on the memory they are operated on: allocation is not only used to get heap memory, but is how files are accessed and threads are created and accessed.

# Planned Features
- **File Systems:**
    - **Tagging File System (TFS):**
    - **Database File System (DBFS):**
    - **Linear Blockchain Access File System (LBAFS) and Archival Block Chain FIle System (ABCFS):**
- **Heterogenous Computing:
    - **Alderlake E and P cores:**
    - **ESP32 Sleep mode execution:**
    - **Tensor and Raytrace core control:**
- **Self-Booting:**

# Build Plan:

# Tools
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
