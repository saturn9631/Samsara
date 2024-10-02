use crate::architecture::MemorySecurityManager;

#[repr(align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; 512],
}

impl MemorySecurityManager for PageTable {
}
