use ctru::{
    prelude::*,
    services::gfx::{Flush, RawFrameBuffer, Screen, Swap},
};

struct FLAPPER {
    y_pos: u32,
    upper_y: u32,
    falling: bool,
}

const MOOMOO_HEIGHT: u32 = 30;
const MOOMOO_WIDTH: u32 = 30;
const FALL_SPEED: u32 = 3;
const BOUNCE_SPEED: u32 = 5;

struct PIPE {
    x_pos: u32,
    upper_x: u32,
    center: u8,
    width: u32,
}

const PIPE_SPEED: u32 = 3;
const PIPE_WIDTH: u32 = 50;

fn main() {
    let apt = Apt::new().unwrap();
    let mut hid = Hid::new().unwrap();
    let mut gfx = Gfx::new().unwrap();
    let mut moomoo = FLAPPER {
        y_pos: 150,
        upper_y: 180,
        falling: true,
    };
    let mut pipe1 = PIPE {
        x_pos: 300,
        upper_x: 300 + PIPE_WIDTH,
        center: 120,
        width: PIPE_WIDTH,
    };
    let mut bounce_prog: u8 = 0;
    while apt.main_loop() {
        hid.scan_input();
        if hid.keys_down().contains(KeyPad::START) {
            break;
        }
        if hid.keys_down().contains(KeyPad::A) {
            moomoo.falling = false;
            bounce_prog = 0;
        }
        if !moomoo.falling {
            bounce_prog += 1;
            if bounce_prog >= 12 {
                moomoo.falling = true;
                bounce_prog = 0;
            }
        }
        movement(&mut moomoo);
        pipe_movement(&mut pipe1);
        {
            let top_screen = gfx.top_screen.get_mut();
            let frame_buffer = top_screen.raw_framebuffer();
            draw_filled_square(
                &frame_buffer,
                50,
                moomoo.y_pos,
                MOOMOO_WIDTH,
                MOOMOO_HEIGHT,
                SQUARE_COLOR,
                true,
            );
            draw_filled_square(
                &frame_buffer,
                pipe1.x_pos,
                0,
                pipe1.width,
                240,
                PIPECOLOR,
                false,
            );
            top_screen.flush_buffers();
            top_screen.swap_buffers();
        }
        gfx.wait_for_vblank();
    }
}

const SQUARE_COLOR_R: u8 = 255;
const SQUARE_COLOR_G: u8 = 0;
const SQUARE_COLOR_B: u8 = 0;

static SQUARE_COLOR: [u8; 3] = [SQUARE_COLOR_B, SQUARE_COLOR_G, SQUARE_COLOR_R];
static BLANK: [u8; 3] = [255, 208, 34];
static PIPECOLOR: [u8; 3] = [63, 181, 45];

fn draw_filled_square(
    frame_buffer: &RawFrameBuffer<'_>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: [u8; 3],
    refresh: bool,
) {
    if refresh {
        draw_filled_square(&frame_buffer, 0, 0, 400, 240, BLANK, false);
    }
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
                frame_buffer_slice[pixel_index..pixel_index + 3].copy_from_slice(&color);
            }
        }
    }
}

fn movement(moomoo: &mut FLAPPER) {
    if moomoo.falling && moomoo.y_pos > 5 {
        moomoo.y_pos -= FALL_SPEED;
        moomoo.upper_y -= FALL_SPEED;
    } else if moomoo.upper_y < 235 && !moomoo.falling {
        moomoo.y_pos += BOUNCE_SPEED;
        moomoo.upper_y += BOUNCE_SPEED;
    }
}

fn pipe_movement(pipe: &mut PIPE) {
    if pipe.width == 0 {
        pipe.x_pos = 400;
    }
    if pipe.x_pos >= PIPE_SPEED && pipe.x_pos <= 400 - PIPE_WIDTH {
        pipe.x_pos -= PIPE_SPEED;
        pipe.upper_x -= PIPE_SPEED;
    } else if pipe.x_pos <= 400 - PIPE_WIDTH {
        pipe.x_pos = 0;
        if pipe.width >= PIPE_SPEED {
            pipe.width -= PIPE_SPEED;
            pipe.upper_x -= PIPE_SPEED;
        } else {
            pipe.width = 0;
            pipe.upper_x = 0;
        }
    } else {
        pipe.upper_x = 400;
        pipe.x_pos -= 3;
        pipe.width += 3;
        if pipe.width >= PIPE_WIDTH {
            pipe.width = PIPE_WIDTH;
            pipe.x_pos = 400 - PIPE_WIDTH;
            pipe.upper_x = pipe.x_pos + PIPE_WIDTH;
        }
    }
}
