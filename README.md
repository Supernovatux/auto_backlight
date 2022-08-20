
# Auto Backlight

Automatically change brightness depending on screen contents. The program takes a screenshot of the primary screen and calculates its average grayscale rgb value and increases/decreases the brightness by a certain value.
## Installation

Git clone the repo and do a cargo build.

```bash
    git clone https://github.com/Supernovatux/auto_backlight
    cd auto_backlight
    cargo build --release
    ./target/release/auto_backlight -h
```
    
## Roadmap

- Improve performance.
    - Effect of converting to grayscale is unknown
    - [This](https://github.com/Cykooz/fast_image_resize) crate does linear image resize faster.
- Add a system-tray widget
- Make it cross platform.

## Related

Here are some related projects

[Gummy](https://github.com/Fushko/gummy)

