use display_info::DisplayInfo;
use fast_image_resize as fr;
use log::{debug, trace};
use std::num::NonZeroU32;

use crate::screen_capture;

pub fn get_value_to_change(lim: u8, brightness: i16,offset:i16) -> i16 {
    debug!("Image brightness {}", brightness);
    ((-2.0 * lim as f64 / 255_f64) * brightness as f64 + lim as f64 + offset as f64) as i16
}

pub fn get_average_brightness(img: Vec<u8>, dsp: DisplayInfo) -> i16 {
    let width = NonZeroU32::new(dsp.width).unwrap();
    let height = NonZeroU32::new(dsp.height).unwrap();
    let src_image = fr::Image::from_vec_u8(width, height, img, fr::PixelType::U8x3).unwrap();
    let dst_width = NonZeroU32::new(160).unwrap();
    let dst_height = NonZeroU32::new(100).unwrap();
    let mut dst_image = fr::Image::new(dst_width, dst_height, fr::PixelType::U8x3);
    let mut dst_view = dst_image.view_mut();
    let mut resizer = fr::Resizer::new(fr::ResizeAlg::Nearest);
    resizer.resize(&src_image.view(), &mut dst_view).unwrap();
    let new = dst_image.into_vec();
    let mut sum = 0;
    let len = new.len() as u64;
    for i in new {
        sum += i as u64;
    }
    (sum / len) as i16
}
pub fn change_calc(lim: u8,offset:i16) -> i16 {
    let screens = screen_capture::Screen::all().unwrap();
    let mut ch = 0;
    for i in screens {
        if i.display_info.is_primary {
            trace!("{:?}", i.display_info);
            let img = match i.capture_raw() {
                Some(img) => img,
                None => {
                    log::error!("Unable to capture screenshot!");
                    continue;
                }
            };
            ch = get_average_brightness(img, i.display_info);
            ch = get_value_to_change(lim, ch,offset);
            break;
        }
    }
    ch
}
