
# Auto Backlight

Automatically change brightness depending on screen contents. The program takes a screenshot of the primary screen and calculates its average RGB value and increases/decreases the brightness by a certain value.
## Installation
Git clone the repo and do a cargo build.

```bash
    git clone https://github.com/Supernovatux/auto_backlight
    cd auto_backlight
    cargo build --release
    cargo install --path=./
    #Make sure ~/.cargo/bin is in your PATH
    auto_backlight -h
```
    
## Roadmap

- Improve performance.
    - Use grayscale and average just a single channel
- Make it cross platform.

## Related

Here are some related projects

[Gummy](https://github.com/Fushko/gummy)

