#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]


mod vga_buffer;



/*Boot using the osdev-rust bootloader
use bootloader::{BootInfo, entry_point};
entry_point!(kernel_main);
#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    vga_buffer::print_something();
    loop {}
}
*/


/*Boot using the osdev-rust bootimage
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();


    loop {}
}*/



/*Boot using custom Multiboot2 header struct
#[no_mangle]
#[repr(C, align(8))]
pub struct Multiboot2 {
    header_magic: u32,
    arch: HeaderTagISA, //placeholder type
    length: u32,
    checksum: u32
}

impl Multiboot2 {
    pub const fn new(arch: HeaderTagISA) -> Self {
        let magic = 0xe85250d6;
        let length = 16;
        let checksum = Self::calc_checksum(magic, arch, length);
        Self {
            header_magic: magic,
            arch,
            length,
            checksum,
        }
    } 

    pub const fn calc_checksum(magic: u32, arch: HeaderTagISA, length: u32) -> u32 {
        (0x100000000 - magic as u64 - arch as u64 - length as u64) as u32
    }

    pub const fn verify_checksum(&self) -> bool {
        let check = Self::calc_checksum(self.header_magic, self.arch, self.length);
        check == self.checksum
    }
}

#[used]
pub static MULTIBOOT_HEADER: Multiboot2 = Multiboot2::new(HeaderTagISA::i386);

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HeaderTagISA {
    i386 = 0,
    MIPS32 = 4,
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}
*/

//Boot Using multiboot defined in a separate assembly file.
use core::arch::global_asm;
global_asm!(include_str!("boot.s"));

#[no_mangle]
//pub extern "c" fn kernel_main() -> ! {
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}
//----------------------------------------------------


use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    loop {}
}


/*
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test()
    }
}
*/
