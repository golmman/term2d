# term2d

## Build, Run Examples

* build everything: `cargo build`
* build library only: `cargo build -p term2d`
* run example `snake`: `cargo run -p snake`

## Notes

* consistently call clone inside functions and make parameter a reference
  * e.g. color, point -> reference!

* Controller
  * half_block_screen
  * primitive_renderer
    * knows half_block_screen
  * image_renderer
    * knows half_block_screen

* rc refcell: https://stackoverflow.com/a/59538577
* Controller
  * Renderer - abstracts draw_rect, draw_circle, draw_image, etc.
    * Display/Canvas - abstracts draw_pixel, draw_char, draw_text, etc.
      * Screen - low level handling of raw terminal

* src
  * lib
  * controller
  * model
    * color
    * rgba
    * point
    * rect
    * image - Image::from(file)
  * view
    * renderer
      * primitive
      * image
    * canvas
      * fullblock
      * halfblock
    * screen
