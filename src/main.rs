fn main() {
    futures::executor::block_on(auto_backlight::init());
}
