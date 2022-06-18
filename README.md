# (Unnamed Image Library)

Pixel perfect image manipulation library.

It's not meant for real-time applications!

It aims to be pixel-perfect.
> For that, it tries not to use floating point values when handling pixels.

It's not GPU-accelerated!
> Other crates that it's using under the hood might use GPUs, but it does not.

It uses [image](https://github.com/image-rs/image) crate for image IO. And [font-kit](https://github.com/servo/font-kit) for rasterizing fonts.


## Features

- [ ] Import/Export
  - [X] Most image formats, thanks to the [image](https://github.com/image-rs/image) crate.
  - [ ] SVG format
  - [ ] Videos
- [X] Pixel by pixel manipulation
- [ ] 32bit RGBA Colors
  - [X] Mono
  - [ ] Gradient colors.
- [ ] Drawing basic shapes
  - [X] Pixel perfect circles
    - [X] Filled
    - [X] Outline
  - [X] Pixel perfect rectangles
    - [X] Filled
    - [X] Outline
  - [ ] Pixel perfect ellipses
    - [X] Filled
    - [ ] Outline
  - [ ] Pixel perfect triangles
    - [X] Filled
    - [ ] Outline
  - [X] Pixel perfect lines
- [ ] Filters
  - It's possible to add filters manually.
  - There's no any built-in filter yet.
- [ ] Transformation
  - [X] Blit
  - [X] Mask
  - [X] Crop
  - [X] Pixel perfect scaling
  - [ ] Pixel perfect rotation
- [X] Fonts
  - [X] Read `.otf` and `.ttf` files
  - [X] Background color
  - [X] Underline
  - [X] Outline
