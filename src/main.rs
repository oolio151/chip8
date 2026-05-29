mod cpu;
mod display;
use cpu::Chip8;
use std::time::Duration;
use std::thread;

fn main() {

    let mut c8: Chip8 = Chip8::new();
    println!("Hello, world!");

    setup_gfx(&mut c8);
    setup_input();

    c8.load_game("pong");

    loop {
        c8.cycle();

        if c8.s_timer() {
            //play sound
        }

        if c8.draw_flag {
            draw(&mut c8);            
        }
        
        c8.set_keys();

        c8.timers();

        thread::sleep(Duration::from_secs_f64(1.0 / 60.0)); //forces 60 fps
    }

}

fn draw(c8: &Chip8) {
    print!("\x1B[2J\x1B[1;1H");
    for i in 0..32 {
        for j in 0..64 {
            print!("{}", if c8.graphics[i * 64 + j] == 1 {"X"} else {" "});
        }
        println!();
    }
}

fn setup_gfx(c8: &mut Chip8) {
    //setting up the font stuff
    for i in 0x050..0x0A0{
        c8.memory[i] = display::FONTSET[i - 0x050];
    }

    //other stuff
}