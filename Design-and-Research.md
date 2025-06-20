# Design
## Outline
- **Core:**
    - **Yggdrasil Language:** the interface between the kernel and the userspace applications, yggdrasil is a domain-specific-language meant to encode system calls and even runtime modules (in the vein of the ebpf in linux)
        - **Object/Collection:** All objects are collections and all collections are objects. Accessing parts of an object can either been done via dot notation or indexing for fields, (if operators are considered the same as methods, then that can be used as an alternative to dot notation for methods.
        - **Operations/Iterator:** Since all objects are collection, all object operators are iterators (it is not decided as to whether operators are separate from methods).
        - **Classes, Interfaces, and Methods:** Classes define the shape of objects, i.e the fields and methods while interfaces define how it appears at different accesses levels (more at msm). Methods are functions stored in the fields of the objects and have a Go-style reciever variable to interact with the object.
    - **Process Schedule Manager:**
        - **Mailboxes and Messages:** A way for threads and procsses to communicate. Either a file or a stream. Can have different buffering/queing destinations and even types off messages.
        - **Supervisor Tree:**  A design enforced at kernel level where the called process/thread passes its errors to a supervisor to handle instead of dealing with it itself. A thread can have either processes or other thread as its supervisor, whereas a process must have another process as its supervisor (although threads can request that a supervisor start a process), with the only parentless process being the PSM. A process is the execution state of a module, and any errors that get pass the PSM are panics that cause a restart or a shutdown.
        - **Execution Context Stack (Language Feature):** Functions and closures are objects that also point to their neighbors. Functions and closures can also be anonymous and passed as values, closures being limited variables where their context is valid. All blocks are closures and all threads are functions allocated to the heap, being the start of the thread's stack.
    - **Memory Security Manager:**
        - **Users and Sessions:** Sessions are an extension to the concept of unix users and groups. Sessions are contexts that are given permissions and ownership of resources. Sessions can be processes, allowing for it to also function as a service user, can be descended from each other allowing for users to double as groups.
        - **Permissions:** Same as in unix, the added permission of visible, allowing for certain files to not only be hidden, but inaccessible. 
        - **Interfaces (Language Feature):** All tables have an interface that defines how another entity is able to interact with it. The interface is a metatable of Properties, and operations.
    - **File Resource Manager:**
        - **Drives:** All devices, phyiscal or virtual, are treated as storage drives with files for interacting with the device organized into a file system.
        - **URL Paths:** All paths are shortened URLs. The scheme is the filesystem type, the authority the hosting drive as well as the containing session and the interface mailbox if applicable, the path in the middle and a optional Yggdrasil statement as the parameter.
        - **Type Hierarchy (Language Feature):** All file paths are types, with subpaths being subpaths, and all types (or possibly most) correspound to paths, sometimes in a virtual or ram based file system.
## Questions and Concerns
# Research
## Topics
## Sources
- [Wikipedia](https://en.wikipedia.org)
    - [Hybrid Kernel](https://en.wikipedia.org/wiki/Hybrid_kernel)
    - [Everything is a File](https://en.wikipedia.org/wiki/Everything_is_a_file)
    - [URL](https://en.wikipedia.org/wiki/URL)
    - [Inodes](https://en.wikipedia.org/wiki/Inode)
