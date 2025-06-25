# Design
## Outline
- **Core:**
    - **Binder:** Essentially the top level part that holds all the of the Yggdrasil definitions and is the is what loads the FRM (which loads the others) and starts the PSM (which starts the other modules). In the case where the kernel self boots, it will also act as the bootloader.
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
- The binder might be a bootloader
- The supervisor tree might not be necessary. Instead it could just be function returns.
# Research
## Topics
## Sources
- [Wikipedia](https://en.wikipedia.org)
    - [Hybrid Kernel](https://en.wikipedia.org/wiki/Hybrid_kernel)
    - [Language-Based System](https://en.wikipedia.org/wiki/Language-based_system)
    - [Everything is a File](https://en.wikipedia.org/wiki/Everything_is_a_file)
    - [URL](https://en.wikipedia.org/wiki/URL)
    - [Inodes](https://en.wikipedia.org/wiki/Inode)
    - [POSIX](https://en.wikipedia.org/wiki/POSIX)
    - [Plan 9](https://en.wikipedia.org/wiki/Plan_9_from_Bell_Labs)
    - [Linux](https://en.wikipedia.org/wiki/Linux)
    - [Linux System](https://en.wikipedia.org/wiki/Linux_kernel)
- [Youtube](https://www.youtube.com/)
    - [Architecture 4021: Introductory UEFI by OpenSecurityTrianing2](https://www.youtube.com/playlist?list=PLUFkSN0XLZ-ltETI20mpXOCdqC8rdven6)
    - [Masen's "How to make a simple boot loader that reads a kernel into memory!](https://www.youtube.com/watch?v=6gLHG0qZ8HA&t=368s)
    - ["EFI Based Operating System Bootloader Series" by ThatOSDev Archive](https://youtube.com/playlist?list=PLdJN-tAX64g6UnGb1rD1wtnd5U6ebGlWd&si=8ibppSkqcbR03eRo)
    - [Queso Fuego's "UEFI Programming in C"](https://youtube.com/playlist?list=PLT7NbkyNWaqZYHNLtOZ1MNxOt8myP5K0p&si=cz463aYuB8WpArCo)
    - [sphaerophoria's "Writing an operating system"](https://youtube.com/playlist?list=PL980gcR1LE3LBuWuSv2CL28HsfnpC4Qf7&si=t5YaCValJDfG7DiE)
    - ["Rust OS" by Uncle Scientist](https://youtube.com/playlist?list=PLib6-zlkjfXkdCjQgrZhmfJOWBk_C2FTY&si=hNXx1tYIztGVczor)
    - [Building an OS by nanobyte](https://youtube.com/playlist?list=PLFjM7v6KGMpiH2G-kT781ByCNC_0pKpPN&si=aXwt_wyrZGCvCQvu)
    - ["Making an OS" by Daedalus Community](https://youtube.com/playlist?list=PLm3B56ql_akNcvH8vvJRYOc7TbYhRs19M&si=uwrciQ-i-nbe54B-)
    - [Ed of Low Level Learning's "rust runs on Everything (no operating system, just rust)"](https://www.youtube.com/watch?v=jZT8APrzvc4&list=TLPQMDcwODIwMjTLmLbfX5NDPA&index=1)
    - [Nir Lichtman's "Making a Bootloader using x86 Assembly](https://www.youtube.com/watch?v=xFrMXzKCXIc&list=TLPQMjcwODIwMjSPG5-G91fv2Q&index=3)
    - [Isaac Harris-Holt](https://www.youtube.com/@IsaacHarrisHolt)
        - [Go for the Impatient Devs](https://www.youtube.com/watch?v=N9Q7icX71hc)
        - [Zig for Impatient Devs](https://www.youtube.com/watch?v=5I4ZkmMS4-0)
    - ["Rust fo the impatient" by No Boilerplate](https://www.youtube.com/watch?v=br3GIIQeefY&t=22s)
    - [Various videos by Core Dumped](https://www.youtube.com/@CoreDumpped)
    - [CPU Scheduling | Chapter-5 | Operating System by Neso Academy](https://www.youtube.com/playlist?list=PLBlnK6fEyqRitWSE_AyyySWfhRgyA-rHk)
    - [DJ Ware](https://www.youtube.com/@CyberGizmo)
        - [Linux Internals](https://www.youtube.com/playlist?list=PLWK00SLo2KcQi1hlP2_allMWeG19MkQa7)
- [Osdev wiki](https://wiki.osdev.org/Expanded_Main_Page)
- [Philipp Opperman's Rust Kernel Blog](https://os.phil-opp.com/minimal-rust-kernel/)
- [Redox OS](https://www.redox-os.org)
- ["Hello world!" without Standard Library by Zenn Dev](https://zenn.dev/zulinx86/articles/rust-nostd-101)
