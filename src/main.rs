use std::time::Duration;
use std::time::Instant;

use chip_8_emulator::emulate;

fn main() -> Result<(), String> {
    let mut interpreter = chip_8_emulator::setup_emulator();
    chip_8_emulator::load_program(&mut interpreter);
    let mut cycle_num: u64 = 0;

    loop {
        let start = Instant::now();
        // The rest of the game loop goes here...
        if emulate(&mut interpreter, cycle_num) {
            chip_8_emulator::render(&mut interpreter);
        }

        chip_8_emulator::handle_input(&mut interpreter);
        let elapsed = start.elapsed();
        if elapsed.as_nanos() < 2_000_000 {
            std::thread::sleep(Duration::new(
                0,
                (2_000_000 - elapsed.as_nanos()).try_into().unwrap(),
            ));
        }
        cycle_num += 1;
    }
}
