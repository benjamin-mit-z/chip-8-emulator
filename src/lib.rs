use rand::Rng;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::EventPump;
use std::env;
use std::fs::File;
use std::io::Read;
use std::process::exit;

const WIDTH: usize = 640;
const HEIGHT: usize = 320;
const SCALE: usize = 10;

#[derive(Copy, Clone, Debug)]
pub struct Pixel {
    rect: Rect,
    is_active: bool,
}

#[derive(Debug)]
pub struct Chip8Cpu {
    programm_counter: u16,
    i: u16,
    stack: Vec<u16>,
    delay_timer: u8,
    sound_timer: u8,
    gen_purpose_registers: [u8; 16],
}

pub struct Interpreter {
    cpu: Chip8Cpu,
    canvas: Canvas<sdl2::video::Window>,
    screen: [[Pixel; HEIGHT / SCALE]; WIDTH / SCALE],
    event_pump: EventPump,
    ram: [u8; 4096],
    input: [bool; 16],
}

impl Pixel {
    fn build() -> Pixel {
        Pixel {
            rect: Rect::new(0, 0, 0, 0),
            is_active: false,
        }
    }
}

pub fn setup_emulator() -> Interpreter {
    let (canv, scr, event_p) = setup_screen();
    let mut interpreter = Interpreter {
        cpu: Chip8Cpu {
            programm_counter: 0x200,
            i: 0,
            stack: Vec::new(),
            delay_timer: 0,
            sound_timer: 0,
            gen_purpose_registers: [0; 16],
        },
        canvas: canv,
        screen: scr,
        event_pump: event_p,
        ram: [0; 4096],
        input: [false; 16],
    };
    let font = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, 0x20, 0x60, 0x20, 0x20, 0x70, 0xF0, 0x10, 0xF0, 0x80, 0xF0,
        0xF0, 0x10, 0xF0, 0x10, 0xF0, 0x90, 0x90, 0xF0, 0x10, 0x10, 0xF0, 0x80, 0xF0, 0x10, 0xF0,
        0xF0, 0x80, 0xF0, 0x90, 0xF0, 0xF0, 0x10, 0x20, 0x40, 0x40, 0xF0, 0x90, 0xF0, 0x90, 0xF0,
        0xF0, 0x90, 0xF0, 0x10, 0xF0, 0xF0, 0x90, 0xF0, 0x90, 0x90, 0xE0, 0x90, 0xE0, 0x90, 0xE0,
        0xF0, 0x80, 0x80, 0x80, 0xF0, 0xE0, 0x90, 0x90, 0x90, 0xE0, 0xF0, 0x80, 0xF0, 0x80, 0xF0,
        0xF0, 0x80, 0xF0, 0x80, 0x80,
    ];
    interpreter.ram[0x50..=0x9F].copy_from_slice(&font);
    interpreter
}

fn setup_screen() -> (
    Canvas<sdl2::video::Window>,
    [[Pixel; HEIGHT / SCALE]; WIDTH / SCALE],
    EventPump,
) {
    let mut grid: [[Pixel; HEIGHT / SCALE]; WIDTH / SCALE] =
        [[Pixel::build(); HEIGHT / SCALE]; WIDTH / SCALE];
    for x in 0..grid.len() {
        for y in 0..grid[x].len() {
            grid[x][y].rect.set_x((x * SCALE).try_into().unwrap());
            grid[x][y].rect.set_y((y * SCALE).try_into().unwrap());
            grid[x][y].rect.set_width(SCALE.try_into().unwrap());
            grid[x][y].rect.set_height(SCALE.try_into().unwrap());
        }
    }
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "chip-8 emulator",
            WIDTH.try_into().unwrap(),
            HEIGHT.try_into().unwrap(),
        )
        .position_centered()
        .build()
        .expect("window creation failed");

    let canvas = window.into_canvas().build().expect("Canvas Builder failed");

    let event_pump = sdl_context.event_pump().unwrap();
    (canvas, grid, event_pump)
}

pub fn load_program(interpreter: &mut Interpreter) {
    let args: Vec<String> = env::args().collect();
    let mut file = File::open(args[1].clone()).expect("Program not found");
    let mut buffer: [u8; 4096] = [0; 4096];
    let size = file.read(&mut buffer).expect("Read Error");
    interpreter.ram[0x200..(0x200 + size)].copy_from_slice(&buffer[0..size]);
}

pub fn render(interpreter: &mut Interpreter) {
    interpreter.canvas.set_draw_color(Color::RGB(0, 0, 0));
    interpreter.canvas.clear();
    interpreter.canvas.set_draw_color(Color::RGB(255, 255, 255));

    for row in interpreter.screen {
        for pixel in row {
            if pixel.is_active {
                let _ = interpreter.canvas.fill_rect(pixel.rect);
            }
        }
    }
    interpreter.canvas.present();
}

fn get_pressed_key(interpreter: &mut Interpreter) -> Option<u8> {
    for i in 0..16 {
        if interpreter.input[i] {
            return Some(i as u8);
        }
    }
    None
}

pub fn handle_input(interpreter: &mut Interpreter) {
    for event in interpreter.event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => exit(0),
            Event::KeyDown { scancode, .. } => match scancode.unwrap() {
                sdl2::keyboard::Scancode::Num1 => interpreter.input[1] = true,
                sdl2::keyboard::Scancode::Num2 => interpreter.input[2] = true,
                sdl2::keyboard::Scancode::Num3 => interpreter.input[3] = true,
                sdl2::keyboard::Scancode::Num4 => interpreter.input[0xC] = true,
                sdl2::keyboard::Scancode::Q => interpreter.input[4] = true,
                sdl2::keyboard::Scancode::W => interpreter.input[5] = true,
                sdl2::keyboard::Scancode::E => interpreter.input[6] = true,
                sdl2::keyboard::Scancode::R => interpreter.input[0xD] = true,
                sdl2::keyboard::Scancode::A => interpreter.input[7] = true,
                sdl2::keyboard::Scancode::S => interpreter.input[8] = true,
                sdl2::keyboard::Scancode::D => interpreter.input[9] = true,
                sdl2::keyboard::Scancode::F => interpreter.input[0xE] = true,
                sdl2::keyboard::Scancode::Z => interpreter.input[0xA] = true,
                sdl2::keyboard::Scancode::X => interpreter.input[0] = true,
                sdl2::keyboard::Scancode::C => interpreter.input[0xB] = true,
                sdl2::keyboard::Scancode::V => interpreter.input[0xF] = true,
                sdl2::keyboard::Scancode::Escape => exit(0),
                _ => (),
            },
            Event::KeyUp { scancode, .. } => match scancode.unwrap() {
                sdl2::keyboard::Scancode::Num1 => interpreter.input[1] = false,
                sdl2::keyboard::Scancode::Num2 => interpreter.input[2] = false,
                sdl2::keyboard::Scancode::Num3 => interpreter.input[3] = false,
                sdl2::keyboard::Scancode::Num4 => interpreter.input[0xC] = false,
                sdl2::keyboard::Scancode::Q => interpreter.input[4] = false,
                sdl2::keyboard::Scancode::W => interpreter.input[5] = false,
                sdl2::keyboard::Scancode::E => interpreter.input[6] = false,
                sdl2::keyboard::Scancode::R => interpreter.input[0xD] = false,
                sdl2::keyboard::Scancode::A => interpreter.input[7] = false,
                sdl2::keyboard::Scancode::S => interpreter.input[8] = false,
                sdl2::keyboard::Scancode::D => interpreter.input[9] = false,
                sdl2::keyboard::Scancode::F => interpreter.input[0xE] = false,
                sdl2::keyboard::Scancode::Z => interpreter.input[0xA] = false,
                sdl2::keyboard::Scancode::X => interpreter.input[0] = false,
                sdl2::keyboard::Scancode::C => interpreter.input[0xB] = false,
                sdl2::keyboard::Scancode::V => interpreter.input[0xF] = false,
                _ => (),
            },
            _ => (),
        }
    }
}

fn fetch_instruction(interpreter: &mut Interpreter) -> u16 {
    let left_part: u8 = interpreter.ram[interpreter.cpu.programm_counter as usize] as u8;
    let right_part: u8 = interpreter.ram[(interpreter.cpu.programm_counter + 1) as usize] as u8;
    ((left_part as u16) << 8) + right_part as u16
}

fn clear_screen(interpreter: &mut Interpreter) {
    for i in 0..interpreter.screen.len() {
        for j in 0..interpreter.screen[i].len() {
            interpreter.screen[i][j].is_active = false;
        }
    }
}

pub fn emulate(interpreter: &mut Interpreter) {
    let instruction: u16 = fetch_instruction(interpreter);
    /*println!("Instruction: {:#06x}\n{:#?}", instruction, interpreter.cpu);
    let mut guess = String::new();
    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");*/
    interpreter.cpu.programm_counter += 2;
    if interpreter.cpu.delay_timer > 0 {
        interpreter.cpu.delay_timer -= 1;
    }
    if interpreter.cpu.sound_timer > 0 {
        interpreter.cpu.sound_timer -= 1;
    } else {
        // println!("beep");
    }
    match instruction >> 12 {
        0 => match (instruction & 0x0F00) >> 8 {
            0x0 => match instruction & 0x00FF {
                0xE0 => clear_screen(interpreter),
                0xEE => {
                    interpreter.cpu.programm_counter =
                        interpreter.cpu.stack.pop().expect("Empty Stack popped")
                }
                _ => eprintln!("Invalid Instruction: {:#06x}", instruction),
            },
            _ => eprintln!("Invalid Instruction: {:#06x}", instruction),
        },
        1 => interpreter.cpu.programm_counter = instruction & 0x0FFF,
        2 => {
            interpreter.cpu.stack.push(interpreter.cpu.programm_counter);
            interpreter.cpu.programm_counter = instruction & 0x0FFF;
        }
        3 => {
            if interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize]
                == (instruction & 0x00FF) as u8
            {
                interpreter.cpu.programm_counter += 2;
            }
        }
        4 => {
            if interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize]
                != (instruction & 0x00FF) as u8
            {
                interpreter.cpu.programm_counter += 2;
            }
        }
        5 => {
            if interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize]
                == interpreter.cpu.gen_purpose_registers[((instruction & 0x00F0) >> 4) as usize]
            {
                interpreter.cpu.programm_counter += 2;
            }
        }
        6 => {
            interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize] =
                (instruction & 0x00FF) as u8;
        }
        7 => {
            interpreter.cpu.gen_purpose_registers[((instruction & 0xF00) >> 8) as usize] =
                interpreter.cpu.gen_purpose_registers[((instruction & 0xF00) >> 8) as usize]
                    .wrapping_add((instruction & 0x00FF) as u8);
        }
        8 => match instruction & 0x000F {
            0 => {
                interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize] =
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x00F0) >> 4) as usize];
            }
            1 => {
                interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize] =
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize]
                        | interpreter.cpu.gen_purpose_registers
                            [((instruction & 0x00F0) >> 4) as usize];
            }
            2 => {
                interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize] =
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize]
                        & interpreter.cpu.gen_purpose_registers
                            [((instruction & 0x00F0) >> 4) as usize];
            }
            3 => {
                interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize] =
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize]
                        ^ interpreter.cpu.gen_purpose_registers
                            [((instruction & 0x00F0) >> 4) as usize];
            }
            4 => {
                let result = interpreter.cpu.gen_purpose_registers
                    [((instruction & 0x0F00) >> 8) as usize] as u16
                    + interpreter.cpu.gen_purpose_registers[((instruction & 0x00F0) >> 4) as usize]
                        as u16;
                interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize] =
                    (result & 0xFF) as u8;
                if result > 0xFF {
                    interpreter.cpu.gen_purpose_registers[15] = 1;
                } else {
                    interpreter.cpu.gen_purpose_registers[15] = 0;
                }
            }
            5 => {
                if interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize]
                    >= interpreter.cpu.gen_purpose_registers[((instruction & 0x00F0) >> 4) as usize]
                {
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize] =
                        interpreter.cpu.gen_purpose_registers
                            [((instruction & 0x0F00) >> 8) as usize]
                            - interpreter.cpu.gen_purpose_registers
                                [((instruction & 0x00F0) >> 4) as usize];
                    interpreter.cpu.gen_purpose_registers[15] = 1;
                } else {
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize] =
                        (0x100
                            - interpreter.cpu.gen_purpose_registers
                                [((instruction & 0x00F0) >> 4) as usize]
                                as u16
                            + interpreter.cpu.gen_purpose_registers
                                [((instruction & 0x0F00) >> 8) as usize]
                                as u16) as u8;
                    interpreter.cpu.gen_purpose_registers[15] = 0x00;
                }
            }
            6 => {
                let val =
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x00F0) >> 4) as usize];
                interpreter.cpu.gen_purpose_registers[((instruction & 0x00F0) >> 4) as usize] =
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x00F0) >> 4) as usize]
                        >> 1;
                interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize] =
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x00F0) >> 4) as usize];
                interpreter.cpu.gen_purpose_registers[15] = val & 0x1;
            }
            7 => {
                if interpreter.cpu.gen_purpose_registers[((instruction & 0x00F0) >> 4) as usize]
                    >= interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize]
                {
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize] =
                        interpreter.cpu.gen_purpose_registers
                            [((instruction & 0x00F0) >> 4) as usize]
                            - interpreter.cpu.gen_purpose_registers
                                [((instruction & 0x0F00) >> 8) as usize];
                    interpreter.cpu.gen_purpose_registers[15] = 1;
                } else {
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize] =
                        (0x100
                            - interpreter.cpu.gen_purpose_registers
                                [((instruction & 0x0F00) >> 8) as usize]
                                as u16
                            + interpreter.cpu.gen_purpose_registers
                                [((instruction & 0x00F0) >> 4) as usize]
                                as u16) as u8;
                    interpreter.cpu.gen_purpose_registers[15] = 0;
                }
            }
            14 => {
                let val =
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x00F0) >> 4) as usize];
                interpreter.cpu.gen_purpose_registers[((instruction & 0x00F0) >> 4) as usize] =
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x00F0) >> 4) as usize]
                        << 1;
                interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize] =
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x00F0) >> 4) as usize];
                interpreter.cpu.gen_purpose_registers[15] = (val >> 7) & 0x1;
            }
            _ => {
                panic!("Invalid Instruction {:#06x}", instruction);
            }
        },
        9 => {
            if interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize]
                != interpreter.cpu.gen_purpose_registers[((instruction & 0x00F0) >> 4) as usize]
            {
                interpreter.cpu.programm_counter += 2;
            }
        }
        10 => interpreter.cpu.i = instruction & 0x0FFF,
        11 => {
            interpreter.cpu.programm_counter =
                (instruction & 0x0FFF) + interpreter.cpu.gen_purpose_registers[0] as u16;
        }
        12 => {
            interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize] =
                rand::thread_rng().gen_range(0..=0xFF) as u8 & (instruction & 0xFF) as u8;
        }
        13 => {
            interpreter.cpu.gen_purpose_registers[15] = 0;
            let x =
                interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize] % 64;
            let y =
                interpreter.cpu.gen_purpose_registers[((instruction & 0x00F0) >> 4) as usize] % 32;
            let n = (instruction & 0x000F) as u8;
            interpreter.cpu.gen_purpose_registers[15] = 0;
            for i in 0..n as usize {
                if y as usize + i > 31 {
                    break;
                }
                let data = interpreter.ram[interpreter.cpu.i as usize + i as usize];
                for j in 0..8 {
                    if x as usize + j > 63 {
                        break;
                    }
                    let bit = data >> 7 - j;
                    if bit & 0x1 == 1 {
                        if interpreter.screen[x as usize + j][y as usize + i].is_active {
                            interpreter.screen[x as usize + j][y as usize + i].is_active = false;
                            interpreter.cpu.gen_purpose_registers[15] = 1;
                        } else {
                            interpreter.screen[x as usize + j][y as usize + i].is_active = true;
                        }
                    }
                }
            }
        }
        14 => match instruction & 0x00FF {
            0x9E => {
                if interpreter.input[interpreter.cpu.gen_purpose_registers
                    [((instruction & 0x0F00) >> 8) as usize]
                    as usize]
                {
                    interpreter.cpu.programm_counter += 2;
                }
            }
            0xA1 => {
                if !(interpreter.input[interpreter.cpu.gen_purpose_registers
                    [((instruction & 0x0F00) >> 8) as usize]
                    as usize])
                {
                    interpreter.cpu.programm_counter += 2;
                }
            }
            _ => panic!("Invalid Instruction: {:#06x}", instruction),
        },
        15 => match instruction & 0xFF {
            0x7 => {
                interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize] =
                    interpreter.cpu.delay_timer
            }
            0x15 => {
                interpreter.cpu.delay_timer =
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize];
            }
            0x18 => {
                interpreter.cpu.sound_timer =
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize];
            }
            0x1E => {
                let result = interpreter.cpu.gen_purpose_registers
                    [((instruction & 0x0F00) >> 8) as usize] as u32
                    + interpreter.cpu.i as u32;
                if result > 0xFFFF {
                    interpreter.cpu.gen_purpose_registers[15] = 1;
                } else {
                    interpreter.cpu.gen_purpose_registers[15] = 0;
                }
                interpreter.cpu.i = (result & 0xFFF) as u16;
            }
            0x0A => match get_pressed_key(interpreter) {
                Some(key) => {
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize] =
                        key;
                }
                None => interpreter.cpu.programm_counter -= 2,
            },
            0x29 => {
                interpreter.cpu.i = interpreter.cpu.gen_purpose_registers
                    [(instruction & 0x000F) as usize] as u16
                    * 5
                    + 0x50;
            }
            0x33 => {
                let num =
                    interpreter.cpu.gen_purpose_registers[((instruction & 0x0F00) >> 8) as usize];
                interpreter.ram[interpreter.cpu.i as usize] = num / 100;
                interpreter.ram[(interpreter.cpu.i + 1) as usize] = num / 10 % 10;
                interpreter.ram[(interpreter.cpu.i + 2) as usize] = num % 10;
            }
            0x55 => {
                let limit = (instruction & 0x0F00) >> 8;
                for i in 0..=limit {
                    interpreter.ram[(interpreter.cpu.i + i) as usize] =
                        interpreter.cpu.gen_purpose_registers[i as usize];
                }
            }
            0x65 => {
                let limit = (instruction & 0x0F00) >> 8;
                for i in 0..=limit {
                    interpreter.cpu.gen_purpose_registers[i as usize] =
                        interpreter.ram[(interpreter.cpu.i + i) as usize];
                }
            }
            _ => panic!("Invalid Instruction: {:#06x}", instruction),
        },
        _ => panic!("Invalid Instruction: {:#06x}", instruction),
    };
    // println!("Instruction: {:#x04}\n{:#?}", instruction, interpreter.cpu);
}
