use chip8::VirtualMachine;
#[cfg(feature = "profile")]
use egui::containers::Window;
use egui::containers::{collapsing_header::CollapsingHeader, TopBottomPanel};
use macroquad::prelude::{coroutines::start_coroutine, *};
#[cfg(feature = "profile")]
use puffin_egui::puffin;
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
const DEFAULT_INSTRUCTIONS_PER_FRAME: u32 = 20;
const TARGET_MS_PER_FRAME: f64 = 1.0 / TARGET_FPS as f64;

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
    #[cfg(feature = "profile")]
    puffin::profile_function!();
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
    #[cfg(feature = "profile")]
    puffin::set_scopes_on(true);

    let mut vm = chip8::VirtualMachine::new();
    let rom = include_bytes!("../../assets/RPS.ch8");
    vm.load_rom(rom);

    let vm = Arc::new(Mutex::new(vm));

    let mut instructions_per_frame = DEFAULT_INSTRUCTIONS_PER_FRAME;
    let mut previous = get_time();
    let mut lag = 0.0;

    loop {
        #[cfg(feature = "profile")]
        puffin::GlobalProfiler::lock().new_frame();

        let current = get_time();
        let elapsed = current - previous;
        previous = current;
        lag += elapsed;

        check_keys(&mut vm.lock().unwrap());

        {
            #[cfg(feature = "profile")]
            puffin::profile_scope!("Update");

            let mut v = vm.lock().unwrap();
            while lag >= TARGET_MS_PER_FRAME {
                lag -= TARGET_MS_PER_FRAME;
                for _ in 0..instructions_per_frame {
                    v.execute_instruction();
                }

                v.decrement_timers();
            }
        }

        clear_background(BLACK);

        egui_macroquad::ui(|ctx| {
            TopBottomPanel::bottom("Bottom Panel")
                .resizable(false)
                .show(ctx, |ui| {
                    CollapsingHeader::new("Config").show(ui, |ui| {
                        let slider =
                            egui::widgets::Slider::new(&mut instructions_per_frame, 5..=500)
                                .text("Instructions Per Frame");
                        ui.add(slider);

                        if ui.button("Load Rom").clicked() {
                            let vm_clone = vm.clone();
                            let fut = async move {
                                if let Some(file_handle) =
                                    rfd::AsyncFileDialog::new().pick_file().await
                                {
                                    let bytes = file_handle.read().await;

                                    vm_clone.lock().unwrap().load_rom(&bytes);
                                }
                            };

                            let fut = NoWakeFuture::new(Box::pin(fut));
                            start_coroutine(fut);
                        }
                    });
                });

            #[cfg(feature = "profile")]
            Window::new("Profiler").constrain(false).show(ctx, |ui| {
                puffin_egui::profiler_ui(ui);
            });
        });

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

        {
            #[cfg(feature = "profile")]
            puffin::profile_scope!("draw");
            egui_macroquad::draw();
        }

        next_frame().await;
    }
}
