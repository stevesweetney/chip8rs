use chip8::VirtualMachine;
use egui::containers::{collapsing_header::CollapsingHeader, TopBottomPanel};
use macroquad::prelude::{coroutines::start_coroutine, *};
use std::sync::{Arc, Mutex};

mod future_util;
mod input_mapping;
use future_util::NoWakeFuture;
use input_mapping::{KeyValue, ACCEPTED_KEYS};

const SCALE_FACTOR: u32 = 24;
const WINDOW_SIZE: (i32, i32) = (
    (chip8::SCREEN_WIDTH as i32) * SCALE_FACTOR as i32,
    (chip8::SCREEN_HEIGHT as i32) * SCALE_FACTOR as i32,
);
const TARGET_FPS: u32 = 60;
const DEFAULT_INSTRUCTIONS_PER_SECOND: u32 = 700;

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

    let vm = Arc::new(Mutex::new(vm));

    let mut instructions_per_second = DEFAULT_INSTRUCTIONS_PER_SECOND;

    loop {
        clear_background(BLACK);

        egui_macroquad::ui(|ctx| {
            TopBottomPanel::bottom("Bottom Panel")
                .resizable(false)
                .show(ctx, |ui| {
                    CollapsingHeader::new("Config").show(ui, |ui| {
                        let slider =
                            egui::widgets::Slider::new(&mut instructions_per_second, 400..=800)
                                .text("Instructions Per Second");
                        ui.add(slider);

                        if ui.button("Load Rom").clicked() {
                            let vm_clone = vm.clone();
                            start_coroutine(async move {
                                let boxed_fut = Box::pin(rfd::AsyncFileDialog::new().pick_file());
                                let file_pick_fut = NoWakeFuture::new(boxed_fut);

                                if let Some(file_handle) = file_pick_fut.await {
                                    let read_fut = NoWakeFuture::new(Box::pin(async move {
                                        file_handle.read().await
                                    }));
                                    let bytes = read_fut.await;

                                    vm_clone.lock().unwrap().load_rom(&bytes);
                                }
                            });
                        }
                    });
                });
        });

        check_keys(&mut vm.lock().unwrap());
        for _ in 0..(instructions_per_second / TARGET_FPS) {
            vm.lock().unwrap().execute_instruction();
        }

        // TODO: Ensure the timers are decremented 60 times a second
        vm.lock().unwrap().decrement_timers();

        for (y, row) in (0u32..).zip(vm.lock().unwrap().screen_rows()) {
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
        egui_macroquad::draw();

        next_frame().await
    }
}
