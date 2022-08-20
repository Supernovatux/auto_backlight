
use std::fs;

use brightness::Brightness;
use futures::{TryStreamExt};
use image::{GenericImageView, DynamicImage, imageops::FilterType};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
pub  async fn change_brightness(change:i16) -> Result<(), brightness::Error> {
    let mut initial_brightness = 0;
    initial_brightness = get_brightness();
    brightness::brightness_devices().try_for_each(|mut dev| async move {
        let value = dev.get().await? as i16;
        let value_new = if change==0 {
            value
        } else if value+change<0 {
            0
        } else if value+change>100 {
            100
        } else {
            value + change
        };
        println!("{}",value_new);
        dev.set(value_new as u32).await.unwrap();
        //println!("Brightness of device {} is {}%", name, value);
        Ok(())
    }).await
}
fn get_brightness()->i16{
    8
}
pub fn get_value_to_change(lim:u8,brightness:u8)->i16 {
    ((-2.0*lim as f64/255 as f64)*brightness as f64 + lim as f64) as i16
}

pub fn get_average_brightness(img:DynamicImage) ->u8 {
    let img = img.resize(159, 100, FilterType::Nearest);
    let img = img.grayscale();
    let idk :Vec<u64>= img.pixels().map(|x| {(x.2[0]as u64+x.2[1] as u64+x.2[2]as u64)/3}).collect();
    let sum:u64 = idk.par_iter().sum();
    (sum/idk.len()as u64) as u8
}