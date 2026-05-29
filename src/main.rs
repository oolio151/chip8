mod cpu;
mod display;
use cpu::Chip8;

fn main() {

    let mut c8: Chip8 = Chip8::new();
    println!("Hello, world!");

    setup_gfx(c8);
    setupInput();

    c8.loadGame("pong");

    loop {
        c8.cycle();
        if c8.draw_flag {
            draw(&mut c8);            
        }
        
        c8.setKeys();
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