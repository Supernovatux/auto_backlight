use auto_backlight::init;

fn main() {
    futures::executor::block_on(init());
}