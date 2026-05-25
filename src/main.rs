mod cpu;
use cpu::Chip8;

fn main() {

    let c8 = Chip8::new();
    println!("Hello, world!");

    setupGfx();
    setupInput();

    c8.init();
    c8.loadGame("pong");

    while true {
        c8.cycle();
        if c8.draw_flag {
            draw();            
        }
        
        c8.setKeys();
    }

}
