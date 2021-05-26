extern crate sdl2;

mod cpu;
mod execution;
mod operation_codes;

use cpu::CPU;
use execution::Execution;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

static SCALE: u32 = 12;
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let location = String::from("roms/snake.ch8");
    let mut cpu = CPU::init();
    let mut execution = Execution::init(&sdl_context, SCALE);

    cpu.load_fontset();
    cpu.load_game(&location);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => execution.handle_key_down(&mut cpu, keycode),
                Event::KeyUp {
                    keycode: Some(keycode),
                    ..
                } => execution.handle_key_up(&mut cpu, keycode),
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 600));
        cpu.execute_cycle(&mut execution);
        if cpu.draw == true {
            execution.draw_canvas(&mut cpu, SCALE);
        }
    }
}
