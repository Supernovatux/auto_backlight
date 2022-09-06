
# Auto Backlight

Automatically change brightness depending on screen contents. The program takes a screenshot of the primary screen and calculates its average RGB value and increases/decreases the brightness by a certain value.
## Installation
### Arch Linux
```bash
cd /tmp
wget https://github.com/Supernovatux/auto_backlight/releases/latest/download/PKGBUILD
makepkg -si
auto-backlight -h
```
### Other distros
Git clone the repo and do a cargo build.

```bash
    git clone https://github.com/Supernovatux/auto_backlight
    cd auto_backlight
    cargo build --release
    cargo install --path=./ --root=/home/$USERNAME/.local
    #Make sure ~/.local/bin is in your PATH
    auto-backlight -h
```
    
## Roadmap

- Improve performance.
- Test on wayland
- Multimonitor support
- Make it cross platform.

## Features
- System tray


## Related

Here are some related projects

[Gummy](https://github.com/Fushko/gummy)

