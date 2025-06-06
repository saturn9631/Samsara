# Primary List
1. Make a boot image
2. Display boot image
3. Load a payload
4. Change from boot mode to runtime mode
# Useful notes
## uefi spec
1. GUID (Globally Unique Identifier): a 16 byte identifier.
2. Tables
    1. Guid Partition Table (GPT): Holds information about the partitions of the disks.
    2. System: stores system info and acesses to std streams (in/out/err).
    3. Boot: holds many system services: memory allocation, loading excutables, and protocols. Can only be used during boot stage (before "exit_boot_services").
    4. runtime: stores (runetime) variables, system time, and virtual-memory mapping. Can be accesed during runtime (after "exit_boot_services").
2. Handles and Protocols: Handles are a pointer that represent resources, whereas protocols provide a way to deal interact with them.
