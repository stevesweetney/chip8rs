use ggez::*;
use input_mapping::KeyValue;
use std::time::{Duration, Instant};

mod chip8;
mod input_mapping;

const MICROSECONDS_PER_FRAME: u64 = 16700;

struct State {
    vm: chip8::VirtualMachine,
}

impl State {
    fn new() -> Self {
        let rom = include_bytes!("../roms/c8_test.c8");
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
            let key_value = value_result.map_err(error::GameError::CustomError)?;

            self.vm.key_state[key_value.0 as usize] = is_down;

            if is_down && self.vm.blocked_on_key_press {
                self.vm.complete_fx0a(key_value.0);
            }
        }

        Ok(())
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        let start = Instant::now();

        self.vm.run_cyle();

        let elapsed = Instant::now().duration_since(start);
        let frame_duration = Duration::from_micros(MICROSECONDS_PER_FRAME);
        timer::sleep(frame_duration.saturating_sub(elapsed));
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> Result<(), GameError> {
        todo!()
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

    let c = conf::Conf::new();
    let (ctx, event_loop) = ContextBuilder::new("chip8", "Steve S.")
        .default_conf(c)
        .build()
        .unwrap();

    event::run(ctx, event_loop, state);
}
