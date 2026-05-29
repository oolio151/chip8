mod cpu;
mod display;
use cpu::Chip8;
use std::time::Duration;
use std::thread;
use std::io;
use minifb::{Window, WindowOptions, Key};

const SCALE: usize = 10;

fn main() {

    let mut c8: Chip8 = Chip8::new();

    let mut input = String::new();
    println!("Enter the game filename w/o extension:");
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    
    //let game: &str = "spaceinvaders";

    setup_gfx(&mut c8);
    //setup_input();

    c8.load_game(input);

    println!("Loaded game {}", input);

    let mut window: Window = Window::new("CHIP-8", 64 * SCALE, 32 * SCALE, WindowOptions::default()).unwrap();
    
    let mut buffer: Vec<u32> = vec![0; 64 * 32 * SCALE * SCALE];

    while window.is_open() {
        
        for _ in 0..10{
            check_input(&mut c8, &window);
            c8.cycle();
        }

        if c8.s_timer() {
            //play sound
        }

        if c8.draw_flag {
            //console_draw(&c8);    
            draw(&c8, &mut window, &mut buffer);
            c8.draw_flag = false;        
        }
        
        //c8.set_keys();

        c8.timers();

        thread::sleep(Duration::from_secs_f64(1.0 / 60.0)); //forces 60 fps
    }

}

fn console_draw(c8: &Chip8) {
    print!("\x1B[2J\x1B[1;1H");
    for i in 0..32 {
        for j in 0..64 {
            print!("{}", if c8.graphics[i * 64 + j] == 1 {"X"} else {" "});
        }
        println!();
    }
} //prints graphics to the console

fn draw(c8: &Chip8, window: &mut Window, buffer: &mut Vec<u32>) {
    for y in 0..32 {
        for x in 0..64 {
            let pixel = c8.graphics[y * 64 + x];
            let color = if pixel == 1 { 0xFFFFFF } else { 0x000000 };
            for sy in 0..SCALE {
                for sx in 0..SCALE {
                    buffer[(y * SCALE + sy) * 64 * SCALE + (x * SCALE + sx)] = color;
                }
            }
        }
    }
    window.update_with_buffer(&buffer, 64 * SCALE, 32 * SCALE).unwrap();
}

fn setup_gfx(c8: &mut Chip8) {
    //setting up the font stuff
    c8.load_fonts(display::FONTSET);

    //other stuff
}

fn check_input(c8: &mut Chip8, window: &Window){
    c8.key[0x0] = window.is_key_down(Key::X);
    c8.key[0x1] = window.is_key_down(Key::Key1);
    c8.key[0x2] = window.is_key_down(Key::Key2);
    c8.key[0x3] = window.is_key_down(Key::Key3);
    c8.key[0x4] = window.is_key_down(Key::Q);
    c8.key[0x5] = window.is_key_down(Key::W);
    c8.key[0x6] = window.is_key_down(Key::E);
    c8.key[0x7] = window.is_key_down(Key::A);
    c8.key[0x8] = window.is_key_down(Key::S);
    c8.key[0x9] = window.is_key_down(Key::D);
    c8.key[0xA] = window.is_key_down(Key::Z);
    c8.key[0xB] = window.is_key_down(Key::C);
    c8.key[0xC] = window.is_key_down(Key::Key4);
    c8.key[0xD] = window.is_key_down(Key::R);
    c8.key[0xE] = window.is_key_down(Key::F);
    c8.key[0xF] = window.is_key_down(Key::V);
}