#![no_std]
#![no_main]

/*use multiboot2_header::builder::Multiboot2HeaderBuilder;
//Tried to make a multiboot2 header using inline assembly.
global_asm!{
    "header_start:",
    "   dd 0xe85250d6", // magic for multiboot2
    "   dd 0", // protected mode i386
    "   dd header_end - header_start", // header length
    "   dd 0x100000000 - (0xe85250d6 + 0 + (header_end - header_start))",
    "header_end",
}

//Tried to make a multiboot header using a struct
#[repr(C, packed)]
struct BootHeader {
    magic: u32,
    length: u32,
    size: u32,
    checksum: u32
}
static BOOT_HEAD: BootHeader = BootHeader 


#[used]
#[no_mangle]
#[link_section = ".text.multiboot2_header"]
static MULTIBOOT2_HDR: [u8; 64] = *include_bytes!("mb2_hdr_dump.bin");
*/

use core::panic::PanicInfo;
use core::arch::global_asm;

global_asm!(include_str!("boot.asm"));


static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2) = 0xb;
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
