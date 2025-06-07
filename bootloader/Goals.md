# Primary List
1. Make a boot image
2. Display boot image
3. Load a payload
4. Change from boot mode to runtime mode
# Useful notes
## uefi basics
1. GUID (Globally Unique Identifier): a 16 byte identifier.
2. Tables
    1. Guid Partition Table (GPT): Holds information about the partitions of the disks. The table contains a header and an entry array that describes each partition on the disk. The GPT is also replicated at the end of the disk to allow for redundancy. The entries in the array consist of a GUID to identify the partition, a partition type GUID to indicate the purpose, the start and end block addresses, and other information.
    2. System: stores system info and acesses to std streams (in/out/err).
    3. Boot: holds many system services: memory allocation, loading excutables, and protocols. Can only be used during boot stage (before "exit_boot_services").
    4. runtime: stores (runetime) variables, system time, and virtual-memory mapping. Can be accesed during runtime (after "exit_boot_services").
3. Handles and Protocols: Handles are a pointer that represent resources, whereas protocols provide a way to deal interact with them.
4. Variables: Stores values using key/value pairs, key consiting of UCS-2 null terminated name plus the vender GUID which serves as a namespace to stop vendor conflicts, and a value stored as a byte array.
    1. Attributes: bit flags that influence access and storage. "BOOTSERVICE_ACCESS" and "RUNTIME_ACCESS" bits both being set allows for access during both boot and runtime, and the NON_VOLATILE allows for the variable to be stored in nvram.
5. System Partition: UEFI's bootable partition, someties called EFI system partition (ESP), organized using a FAT (file allocation table) filesystem and identified with the GUID "c12a7328-f81f-11d2-ba4b-00a0c93ec93b". UEFI will try to boot from the special path "/EFI/BOOT/BOOT<ARCH>.efi" where "<ARCH>" is a stand in for the architecture: e.g. "EFI/BOOT/BOOTX64.efi" for x86_64, or it boot from a diffeent place if certain variables are modified.
## rust-uefi
- proto
    - SimpleFileSystem struct
- boot module
    - get_image_file_system function
- SystemTables
