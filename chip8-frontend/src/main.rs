use chip8::VirtualMachine;
use macroquad::prelude::*;

mod input_mapping;
use input_mapping::{KeyValue, ACCEPTED_KEYS};

const SCALE_FACTOR: u32 = 24;
const WINDOW_SIZE: (f32, f32) = (
    (chip8::SCREEN_WIDTH as f32) * SCALE_FACTOR as f32,
    (chip8::SCREEN_HEIGHT as f32) * SCALE_FACTOR as f32,
);
const TARGET_FPS: u32 = 60;
const INSTRUCTIONS_PER_SECOND: u32 = 700;
const INSTRUCTIONS_PER_FRAME: u32 = INSTRUCTIONS_PER_SECOND / TARGET_FPS;

fn handle_key_event(vm: &mut VirtualMachine, keycode: KeyCode, is_down: bool) {
    let value_result: Result<KeyValue, _> = KeyValue::try_from(keycode);

    match value_result {
        Ok(key_value) => {
            if vm.blocked_on_key_press && vm.key_state[key_value.0 as usize] && !is_down {
                vm.complete_fx0a(key_value.0);
            }

            vm.key_state[key_value.0 as usize] = is_down;
        }
        Err(message) => eprintln!("Error: {}", message),
    }
}

fn check_keys(vm: &mut VirtualMachine) {
    for keycode in ACCEPTED_KEYS {
        if is_key_down(keycode) {
            handle_key_event(vm, keycode, true);
        } else if is_key_released(keycode) {
            handle_key_event(vm, keycode, false);
        }
    }
}

#[macroquad::main("Chip8")]
async fn main() {
    let _vm = chip8::VirtualMachine::new();

    loop {
        clear_background(BLACK);

        next_frame().await
    }
}
