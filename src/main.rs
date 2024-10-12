#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]



mod vga_buffer;

#![reexport_test_harness_main = "test_main"]

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main()

    loop {}
}


use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    loop {}
}


#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test()
    }
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
}
