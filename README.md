# (Unnamed Image Library)

Pixel perfect image manipulation library.

It's not meant for real-time applications!

It aims to be pixel-perfect.
> For that, it tries not to use floating point values when handling pixels.

It's not GPU-accelerated!
> Other crates that it's using under the hood might use GPUs, but it does not.

It uses [image](https://github.com/image-rs/image) crate for image IO. And [font-kit](https://github.com/servo/font-kit) for rasterizing fonts.

---

- Why binary, not library?
  - It should be library,,,
  - I should fix that someday,,,