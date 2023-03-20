use chip8::VirtualMachine;
use macroquad::prelude::*;

mod input_mapping;
use input_mapping::KeyValue;

const SCALE_FACTOR: u32 = 24;
const WINDOW_SIZE: (f32, f32) = (
    (chip8::SCREEN_WIDTH as f32) * SCALE_FACTOR as f32,
    (chip8::SCREEN_HEIGHT as f32) * SCALE_FACTOR as f32,
);
const TARGET_FPS: u32 = 60;
const INSTRUCTIONS_PER_SECOND: u32 = 700;
const INSTRUCTIONS_PER_FRAME: u32 = INSTRUCTIONS_PER_SECOND / TARGET_FPS;

#[macroquad::main("Chip8")]
async fn main() {
    let _vm = chip8::VirtualMachine::new();

    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
