use std::time::Duration;

use chip_8_emulator::emulate;

fn main() -> Result<(), String> {
    let mut interpreter = chip_8_emulator::setup_emulator();
    chip_8_emulator::load_program(&mut interpreter);

    loop {
        // The rest of the game loop goes here...
        emulate(&mut interpreter);
        chip_8_emulator::render(&mut interpreter);

        chip_8_emulator::handle_input(&mut interpreter);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 480));
    }
}
