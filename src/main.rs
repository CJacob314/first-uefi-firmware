#![no_main]
#![no_std]

mod status_error;
use status_error::StatusError as StatErr;

use core::ptr::write_volatile;
use log::{info, error};
use uefi::prelude::*;
use uefi::boot::{open_protocol_exclusive, get_handle_for_protocol};
use uefi::proto::console::{gop::{BltPixel, GraphicsOutput}, pointer::{Pointer}};
use tinybmp::Bmp;
use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
// use embedded_graphics_core::geometry::{OriginDimensions, Size};

#[entry]
fn _main() -> Status {

    match main() {
        Ok(_) => Status::SUCCESS,
        Err(StatErr(stat, err)) => {
            if let Some(err) = err {
                error!("Error: {err}");
            }
            boot::stall(FIVE_SECONDS);
            stat 
        }
    }
}

fn main() -> Result<(), StatErr> {
    uefi::helpers::init().unwrap();
    info!("Welcome to my first UEFI project!");
    boot::stall(TWO_SECONDS);

    let graphics_handle = get_handle_for_protocol::<GraphicsOutput>().map_err(|e| (Status::DEVICE_ERROR, e))?;
    let mut graphics_protocol = open_protocol_exclusive::<GraphicsOutput>(graphics_handle).map_err(|e| (Status::DEVICE_ERROR, e))?;

    if let Err(status) = draw(&mut graphics_protocol) {
        return Err(StatErr(status, None));
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
            write_volatile(frame_buff_ptr, blt_pixel);
            frame_buff_ptr = frame_buff_ptr.add(1);
        }
    }

    Ok(())
}

const FIVE_SECONDS: usize = 5_000_000;
const TWO_SECONDS: usize = 2_000_000;

