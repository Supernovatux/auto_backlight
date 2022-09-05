use display_info::DisplayInfo;
use xcb::x::{Drawable, GetImage, ImageFormat};

fn capture_raw(x: i32, y: i32, width: u32, height: u32) -> Option<Vec<u8>> {
    let (conn, index) = xcb::Connection::connect(None).ok()?;

    let setup = conn.get_setup();
    let screen = setup.roots().nth(index as usize)?;

    let get_image_cookie = conn.send_request(&GetImage {
        format: ImageFormat::ZPixmap,
        drawable: Drawable::Window(screen.root()),
        x: x as i16,
        y: y as i16,
        width: width as u16,
        height: height as u16,
        plane_mask: u32::MAX,
    });

    let get_image_reply = conn.wait_for_reply(get_image_cookie).ok()?;
    Some(Vec::from(get_image_reply.data()))
}
pub fn xorg_capture_screen_raw(display_info: &DisplayInfo) -> Option<Vec<u8>> {
    let x = ((display_info.x as f32) * display_info.scale_factor) as i32;
    let y = ((display_info.y as f32) * display_info.scale_factor) as i32;
    let width = ((display_info.width as f32) * display_info.scale_factor) as u32;
    let height = ((display_info.height as f32) * display_info.scale_factor) as u32;

    capture_raw(x, y, width, height)
}
