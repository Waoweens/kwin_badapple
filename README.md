# kwin_badapple

Bad Apple!! but with nested Wayland compositors.
why use a graphics library when you can spawn a window manager for every pixel?

Adapted from my [original Python script](https://github.com/Waoweens/experiments/blob/main/badapple/badapple.py), which could not keep up so I had to use a faster language.

This is my first real Rust project, so i'm sorry in advance.

## Usage
Due to the large number of images, it is not included in the repository. You can download it from [archive.org](https://archive.org/details/bad_apple_is.7z). Point the program to the image_sequence directory and bad_apple.wav.

### Format
`kwin_badapple [OPTIONS] <image_sequence> <audio>`

### Example
```sh
$ kwin_badapple /path/to/image_sequence /path/to/bad_apple.wav
```

### Options
- `--fps` FPS as a divisor of 29.97. Default: `2` for 14.985 FPS.
- `--parent-width` and `--parent-height` to set the parent compositor size. Should be 4:3. Default: `640` and `480` respectively.
- `--pixel-size` size of each child compositor. Should fit the parent size. Default: `32` for 20x15 at 640x480.

## Building
this section will be updated later