// import other modules
#![no_std]
#![no_main]
/*mod boot {
    use core::arch::asm;
    use core::arch::gloal_asm;
    global_asm!(
        ".section .text._start"
    )

    
}*/


use core::panic::PanicInfo;
use core::arch::asm;

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
