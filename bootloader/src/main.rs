#![no_std]
#![no_main]

use log::info;
//use uefi::prelude::*;
use uefi::Status;
//use uefi::boot::get_image_file_system;
use uefi::proto::media::fs::SimpleFileSystem;


#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("----------------Start of UEFI----------------");
    boot::stall(10_000_000);

    efi_setup();

    info!("----------------End of UEFI----------------");
    boot::stall(10_000_000);

    Status::SUCCESS

}

fn read_file() -> File {
    let loaded_image = 
}


