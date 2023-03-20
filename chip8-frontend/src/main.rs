use chip8::VirtualMachine;
use macroquad::prelude::*;

mod input_mapping;
use input_mapping::{KeyValue, ACCEPTED_KEYS};

const SCALE_FACTOR: u32 = 24;
const WINDOW_SIZE: (i32, i32) = (
    (chip8::SCREEN_WIDTH as i32) * SCALE_FACTOR as i32,
    (chip8::SCREEN_HEIGHT as i32) * SCALE_FACTOR as i32,
);
const TARGET_FPS: u64 = 60;
const INSTRUCTIONS_PER_SECOND: u64 = 700;
const INSTRUCTIONS_PER_FRAME: u64 = INSTRUCTIONS_PER_SECOND / TARGET_FPS;

fn window_conf() -> Conf {
    Conf {
        window_title: "Chip8rs".to_owned(),
        window_width: WINDOW_SIZE.0,
        window_height: WINDOW_SIZE.1,
        ..Default::default()
    }
}

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

#[macroquad::main(window_conf)]
async fn main() {
    let mut vm = chip8::VirtualMachine::new();
    let rom = include_bytes!("../chip8-test-suite.ch8");
    vm.load_rom(rom);

    loop {
        clear_background(BLACK);

        check_keys(&mut vm);
        for _ in 0..INSTRUCTIONS_PER_FRAME {
            vm.execute_instruction();
        }

        // TODO: Ensure the timers are decremented 60 times a second
        vm.decrement_timers();

        for (y, row) in (0u32..).zip(vm.screen_rows()) {
            for (x, _) in (0u32..).zip(row).filter(|(_, p)| **p != 0) {
                draw_rectangle(
                    (x * SCALE_FACTOR) as f32,
                    (y * SCALE_FACTOR) as f32,
                    SCALE_FACTOR as f32,
                    SCALE_FACTOR as f32,
                    WHITE,
                );
            }
        }

        next_frame().await
    }
}
