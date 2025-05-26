#![no_std]
#![no_main]

#[entry]
fn main() -> Status {
    efi_setup();
    enter_runtime();
}

fn enter_runtime() {
}

fn efi_setup() {
}
