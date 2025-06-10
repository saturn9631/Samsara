#![no_main]
#![no_std]

use log::info;
use uefi::{Status, entry, Identify, Result};
use uefi::proto::BootPolicy;
use uefi::proto::device_path::DevicePath;
use uefi::proto::device_path::text::{AllowShortcuts, DevicePathToText, DisplayOnly};
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::media::load_file::LoadFile;
use uefi::proto::media::file::FileInfo;
use uefi::boot::{SearchType, stall, open_protocol_exclusive, image_handle, locate_handle_buffer, get_image_file_system, exit_boot_services};
//use uefi::data_types::PoolString;
use uefi::proto::device_path::text::PoolString;
 
#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    //let file = getFile("");
    let error_code = printImage();
    info!("\"getFile\" returned with error code {}.", error_code);

    info!("----------------Done----------------");
    stall(30_000_000);

    Status::SUCCESS
}

fn printImage() -> u8 {
    let image = match open_protocol_exclusive::<LoadedImage>(image_handle()) {
        Ok(some_image) => some_image,
        Err(error) => {
            info!("Could not get image due to {} error.", error);
            return 1;
        },
    };
    //let device_path_handle = locate_handle_buffer(SearchType::ByProtocol(&DevicePath::GUID));
    let path_reference = match image.file_path() {
        None => {
            info!("Could not get image file path (it is possible it does not have one).");
            return 2;
        },
        Some (Path) => Path,
    };
    let image_name = match path_reference.to_string(DisplayOnly(true), AllowShortcuts(true)) { 
        Ok(name) => name,
        Err(error) => {
            info!("Could not open path due to {} error", error);
            return 3;
        },
    };
    info!("The current file path is: {}", image_name);

    0
}

fn getFile() {
    let mut filesystem = match get_image_file_system(image_handle()) {
        Ok(fs) => fs,
        Err(error) => {
            info!("Could not open path due to {} error", error);
            return;
        },
    };
    let root = match filesystem.open_volume() {
        Ok(directory) => directory,
        Err(error) => {
            info!("Could not open path due to {} error", error);
            return;
        },
    };
}
