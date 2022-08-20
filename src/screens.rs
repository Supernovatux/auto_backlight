use image::{imageops::FilterType, DynamicImage, GenericImageView};
use log::debug;
use std::fs;

pub fn get_value_to_change(lim: u8, brightness: i16) -> i16 {
    ((-2.0 * lim as f64 / 255_f64) * brightness as f64 + lim as f64) as i16
}

pub fn get_average_brightness(img: DynamicImage) -> i16 {
    let img = img.resize(159, 100, FilterType::Nearest);
    //Not sure if this is done properly but it works!
    let img = img.grayscale();
    //Why does grayscale have RGBA. shouldn't two channels be sufficient?
    let idk: Vec<u64> = img
        .pixels()
        .map(|x| (x.2[0] as u64 + x.2[1] as u64 + x.2[2] as u64) / 3)
        .collect();
    let sum: u64 = idk.iter().sum();
    (sum / idk.len() as u64) as i16
}
pub fn change_calc(lim: u8) -> i16 {
    let screens = screenshots::Screen::all().unwrap();
    let mut ch = 0;
    for i in screens {
        if i.display_info.is_primary {
            debug!("{:?}", i.display_info);
            let img = i.capture().unwrap();
            fs::write("tmp.png", &img.buffer()).unwrap();
            let img = image::open("./tmp.png").unwrap();
            ch = get_average_brightness(img);
            debug!("To brightness {}", ch);
            ch = get_value_to_change(lim, ch);
            break;
        }
    }
    ch as i16
}
