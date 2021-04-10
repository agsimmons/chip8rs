use minifb::{Scale, Window, WindowOptions};
use std::time::Duration;

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;
const FRAME_TIME: Duration = Duration::from_micros(16600);
const COLOR_EMPTY: u32 = 0x000000;
const COLOR_FILLED: u32 = 0xFFFFFF;

pub struct Display {
    pixels: [u32; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    pub window: Window,
}

impl Display {
    pub fn new() -> Display {
        let window_options = WindowOptions {
            scale: Scale::X16,
            ..WindowOptions::default()
        };

        let mut window = Window::new(
            "Chip8-rs - ESC to exit",
            DISPLAY_WIDTH,
            DISPLAY_HEIGHT,
            window_options,
        )
        .unwrap_or_else(|err| {
            panic!("Could not create window: {}", err);
        });

        window.limit_update_rate(Some(FRAME_TIME));

        Display {
            pixels: [COLOR_EMPTY; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            window: window,
        }
    }

    /// Clears the display
    pub fn clear(&mut self) {
        self.pixels.iter_mut().for_each(|x| *x = COLOR_EMPTY);
    }

    pub fn update(&mut self) {
        self.window
            .update_with_buffer(&self.pixels, DISPLAY_WIDTH, DISPLAY_HEIGHT)
            .unwrap();
    }

    /// Wraps coordinates around the display in both x and y
    pub fn get_wrapped_coordinates(x: usize, y: usize) -> (usize, usize) {
        let x = x.rem_euclid(DISPLAY_WIDTH);
        let y = y.rem_euclid(DISPLAY_HEIGHT);

        (x, y)
    }

    /// Given the coordinates of a pixel on the display, calculate the index of
    // the pixel array. This must be provided with a pre-wrapped value. See
    // get_wrapped_coordinates
    fn coordinate_to_index(x: usize, y: usize) -> usize {
        x + (y * DISPLAY_WIDTH)
    }

    /// Draws sprite at specified coordinate
    /// The return value will be true if this draw operation causes any pixel
    /// to be erased
    pub fn draw_sprite(&mut self, x: usize, y: usize, sprite_data: &[u8]) -> bool {
        println!("Drawsprite at ({}, {})", x, y);
        println!("Sprite Data: {:02X?}", sprite_data);

        let mut pixels_erased = false;
        for (i, line) in sprite_data.iter().enumerate() {
            let local_y = y + i;
            for j in 0..8 {
                let local_x = x + j;
                let (wrapped_x, wrapped_y) = Display::get_wrapped_coordinates(local_x, local_y);
                let pixel_index = Display::coordinate_to_index(wrapped_x, wrapped_y);

                // The selector is a one bit mask that is used to extract the
                // value of the sprite at this coordinate
                let selector = 0b1000_0000u8 >> j;

                let sprite_pixel_value = (line & selector) >> (7 - j);
                let display_pixel_value = self.pixels[pixel_index]; // Maybe make reference

                if sprite_pixel_value == 0x0 && display_pixel_value == COLOR_EMPTY {
                    self.pixels[pixel_index] = COLOR_EMPTY;
                } else if sprite_pixel_value == 0x0 && display_pixel_value == COLOR_FILLED {
                    self.pixels[pixel_index] = COLOR_FILLED;
                } else if sprite_pixel_value == 0x1 && display_pixel_value == COLOR_EMPTY {
                    self.pixels[pixel_index] = COLOR_FILLED;
                } else if sprite_pixel_value == 0x1 && display_pixel_value == COLOR_FILLED {
                    // I'm pretty sure that the only way this operation would
                    // erase an existing pixel is if both the sprite value is
                    // filled and the existing display value is also filled,
                    // therefor, I've added a check for this case.
                    pixels_erased = true;
                    self.pixels[pixel_index] = COLOR_EMPTY;
                } else {
                    panic!("No matching condition for drawing pixel. This shouldn't be possible");
                }
            }
        }

        pixels_erased
    }
}
