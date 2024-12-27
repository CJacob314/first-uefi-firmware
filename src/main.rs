#![no_main]
#![no_std]

use core::ptr::write_volatile;
use log::{info, error};
use uefi::prelude::*;
use uefi::boot::{open_protocol_exclusive, get_handle_for_protocol};
use uefi::proto::console::gop::{BltPixel, GraphicsOutput};
use tinybmp::Bmp;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
// use embedded_graphics_core::geometry::{OriginDimensions, Size};

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Welcome to my first UEFI project!");
    boot::stall(TWO_SECONDS);

    let graphics_handle = match get_handle_for_protocol::<GraphicsOutput>() {
        Ok(handle) => handle,
        Err(e) => {
            error!("Error getting graphics handle: {e}");
            boot::stall(FIVE_SECONDS);
            return Status::DEVICE_ERROR;
        }
    };

    let mut graphics_protocol = match open_protocol_exclusive::<GraphicsOutput>(graphics_handle) {
        Ok(prot) => prot,
        Err(e) => {
            error!("Error getting graphics protocol: {e}");
            boot::stall(FIVE_SECONDS);
            return Status::DEVICE_ERROR;
        }
    };

    if let Err(status) = draw(&mut graphics_protocol) {
        return status;
    }

    loop {
        core::hint::spin_loop();
    }
}

/// Draws the image inside "$PROJECT_ROOT/boot_image.bmp" at compile-time to `gop`
fn draw(gop: &mut GraphicsOutput) -> Result<(), Status> {
    // Get BMP from the EFI image
    let data = include_bytes!("../boot_image.bmp");
    let bmp: Bmp<Rgb888> = Bmp::from_slice(data).map_err(|_| Status::LOAD_ERROR)?;

    // Width and height
    // let Size { width, height } = bmp.size();

    let mut frame_buff = gop.frame_buffer();
    let mut frame_buff_ptr = frame_buff.as_mut_ptr() as *mut BltPixel;

    // Write pixels to frame buffer, one-by-one
    for pixel in bmp.pixels() {
        let c = pixel.1;
        let blt_pixel = BltPixel::new(c.r(), c.g(), c.b());
        unsafe { 
            write_volatile(frame_buff_ptr, blt_pixel);
            frame_buff_ptr = frame_buff_ptr.add(1);
        }
    }

    Ok(())
}

const FIVE_SECONDS: usize = 5_000_000;
const TWO_SECONDS: usize = 2_000_000;

