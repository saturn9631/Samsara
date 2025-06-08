#![no_main]
#![no_std]

use log::info;
use uefi::boot::{self, SearchType, stall, open_protocol_exclusive, image_handle, locate_handle_buffer};
use uefi::prelude::*;
use uefi::proto::device_path::text::{
    AllowShortcuts, DevicePathToText, DisplayOnly,
};
use uefi::proto::loaded_image::LoadedImage;
use uefi::{Identify, Result};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    print_image_path().unwrap();

    stall(15_000_000);
    Status::SUCCESS
}

fn print_image_path() -> Result {
    let loaded_image =
        open_protocol_exclusive::<LoadedImage>(image_handle())?;

    let device_path_to_text_handle = *locate_handle_buffer(
        SearchType::ByProtocol(&DevicePathToText::GUID),
    )? //DevicePath may not have a guid
    .first()
    .expect("DevicePathToText is missing");

    let device_path_to_text = open_protocol_exclusive::<DevicePathToText>(
        device_path_to_text_handle,
    )?;

    let image_device_path =
        loaded_image.file_path().expect("File path is not set");
    let image_device_path_text = device_path_to_text
        .convert_device_path_to_text(
            image_device_path,
            DisplayOnly(true),
            AllowShortcuts(false),
        )
        .expect("convert_device_path_to_text failed");

    info!("Image path: {}", &*image_device_path_text);
    Ok(())
}

