use ggez::*;
use input_mapping::KeyValue;

mod input_mapping;

const SCALE_FACTOR: u32 = 24;
const WINDOW_SIZE: (f32, f32) = (
    (chip8::SCREEN_WIDTH as f32) * SCALE_FACTOR as f32,
    (chip8::SCREEN_HEIGHT as f32) * SCALE_FACTOR as f32,
);
const TARGET_FPS: u32 = 60;
const INSTRUCTIONS_PER_SECOND: u32 = 700;
const INSTRUCTIONS_PER_FRAME: u32 = INSTRUCTIONS_PER_SECOND / TARGET_FPS;

struct State {
    vm: chip8::VirtualMachine,
}

impl State {
    fn new() -> Self {
        let rom = include_bytes!("./tests/chip8-test-suite.ch8");
        let mut vm = chip8::VirtualMachine::new();
        vm.load_rom(rom);
        Self { vm }
    }

    fn handle_key_event(
        &mut self,
        input: input::keyboard::KeyInput,
        is_down: bool,
    ) -> Result<(), GameError> {
        if let Some(keycode) = input.keycode {
            let value_result: Result<KeyValue, _> = keycode.try_into();

            match value_result {
                Ok(key_value) => {
                    if self.vm.blocked_on_key_press
                        && self.vm.key_state[key_value.0 as usize]
                        && !is_down
                    {
                        self.vm.complete_fx0a(key_value.0);
                    }

                    self.vm.key_state[key_value.0 as usize] = is_down;
                }
                Err(message) => eprintln!("Error: {}", message),
            }
        }

        Ok(())
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        while ctx.time.check_update_time(TARGET_FPS) {
            for _ in 0..INSTRUCTIONS_PER_FRAME {
                self.vm.execute_instruction();
            }
            self.vm.decrement_timers();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        for (y, row) in self.vm.screen_rows().enumerate() {
            for (x, _) in row.iter().enumerate().filter(|(_, p)| **p != 0) {
                let rect = graphics::Rect::new_i32(
                    x as i32 * SCALE_FACTOR as i32,
                    y as i32 * SCALE_FACTOR as i32,
                    SCALE_FACTOR as i32,
                    SCALE_FACTOR as i32,
                );

                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(rect)
                        .color(graphics::Color::WHITE),
                );
            }
        }

        canvas.finish(ctx)?;

        ggez::timer::yield_now();

        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: input::keyboard::KeyInput,
        _repeated: bool,
    ) -> Result<(), GameError> {
        self.handle_key_event(input, true)
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        input: input::keyboard::KeyInput,
    ) -> Result<(), GameError> {
        self.handle_key_event(input, false)
    }
}

fn main() {
    let state = State::new();

    let c = conf::Conf::new()
        .window_mode(conf::WindowMode::default().dimensions(WINDOW_SIZE.0, WINDOW_SIZE.1));
    let (ctx, event_loop) = ContextBuilder::new("chip8", "Steve S.")
        .default_conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}
