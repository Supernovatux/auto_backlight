use fast_image_resize as fr;
use image::{DynamicImage, GenericImageView};
use log::{debug, info, trace};
use std::num::NonZeroU32;

use crate::cli_parser::get_offset;

pub fn get_value_to_change(lim: u8, brightness: i16) -> i16 {
    debug!("Image brightness {}", brightness);
    ((-2.0 * lim as f64 / 255_f64) * brightness as f64 + lim as f64 + get_offset() as f64) as i16
}

pub fn get_average_brightness(img: DynamicImage) -> i16 {
    let width = NonZeroU32::new(img.width()).unwrap();
    let height = NonZeroU32::new(img.height()).unwrap();
    let src_image = fr::Image::from_vec_u8(
        width,
        height,
        img.to_rgba8().into_raw(),
        fr::PixelType::U8x4,
    )
    .unwrap();
    let dst_width = NonZeroU32::new(160).unwrap();
    let dst_height = NonZeroU32::new(100).unwrap();
    let mut dst_image = fr::Image::new(dst_width, dst_height, fr::PixelType::U8x4);
    let mut dst_view = dst_image.view_mut();
    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Nearest);
    resizer.resize(&src_image.view(), &mut dst_view).unwrap();
    let new = image::RgbaImage::from_vec(dst_width.get(), dst_height.get(), dst_image.into_vec())
        .unwrap();
    let img = image::DynamicImage::ImageRgba8(new);
    let img = img.grayscale();
    let idk: Vec<u32> = img
        .pixels()
        .map(|x| (x.2[0] as u32 + x.2[1] as u32 + x.2[2] as u32) / 3)
        .collect();
    let sum: u32 = idk.iter().sum();
    (sum / idk.len() as u32) as i16
}
pub fn change_calc(lim: u8) -> i16 {
    let screens = screenshots::Screen::all().unwrap();
    let mut ch = 0;
    for i in screens {
        if i.display_info.is_primary {
            trace!("{:?}", i.display_info);
            let img = i.capture().unwrap();
            let img = image::load_from_memory(img.buffer()).unwrap();
            ch = get_average_brightness(img);
            ch = get_value_to_change(lim, ch);
            info!("Result of ch {}", ch);
            break;
        }
    }
    ch as i16
}
