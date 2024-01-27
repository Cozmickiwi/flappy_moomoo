use ctru::{
    prelude::*,
    services::gfx::{Flush, RawFrameBuffer, Screen, Swap},
};

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let mut gfx = Gfx::new().unwrap();
    let top_screen = gfx.top_screen.get_mut();
    top_screen.swap_buffers();
    let frame_buffer = top_screen.raw_framebuffer();
    //    let mut old_keys = KeyPad::empty();
    draw_filled_square(frame_buffer, 300, 100, 50, 15);
    top_screen.flush_buffers();
    top_screen.swap_buffers();
    while apt.main_loop() {
        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
        gfx.wait_for_vblank();
    }
}

const SQUARE_COLOR_R: u8 = 255;
const SQUARE_COLOR_G: u8 = 0;
const SQUARE_COLOR_B: u8 = 0;

static SQUARE_COLOR: [u8; 3] = [SQUARE_COLOR_B, SQUARE_COLOR_G, SQUARE_COLOR_R];

fn draw_filled_square(frame_buffer: RawFrameBuffer<'_>, x: u32, y: u32, width: u32, height: u32) {
    let frame_buffer_slice = unsafe {
        std::slice::from_raw_parts_mut(
            frame_buffer.ptr,
            ((frame_buffer.height * frame_buffer.width) * 3) as usize,
        )
    };
    for i in 0..height {
        for a in 0..width {
            let new_x = x + a;
            let new_y = y + i;
            if new_x < frame_buffer.height as u32 && new_y < frame_buffer.width as u32 {
                let pixel_index = ((new_x) * frame_buffer.width as u32 + (new_y)) as usize * 3;
                frame_buffer_slice[pixel_index..pixel_index + 3].copy_from_slice(&SQUARE_COLOR);
            }
        }
    }
}