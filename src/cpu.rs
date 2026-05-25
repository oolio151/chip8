pub struct Chip8 {
    memory: [u8; 4096],
    //mwemory map
    //0x000 to 0x1FF is the interpreter
    //0x050 to 0x0A0 used for built in 4x5 pixel font set
    //0x200 - 0xFFF is used for program ROM and work RAM
    opcode: i16,
    registers: [u8; 16],
    I: i16,
    pc: i16,
    graphics: [u8; 2048],
    delay_timer: u8,
    sound_timer: u8,

    stack: [i16; 16],
    sp: i16,

    key: [i16; 16],

    draw_flag: bool
} 

impl Chip8 {
    pub fn new() -> Self {
        Self {
            memory: [0u8; 4096],
            opcode: 0,
            registers: [0u8; 16],
            I: 0,
            pc: 0x200, 
            graphics: [0u8; 2048],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0i16; 16],
            sp: 0,
            key: [0i16; 16],
            draw_flag: false,
        }
    }


    pub fn init() {

    }

    pub fn cycle(&mut self) {
        let opcode = (self.memory[self.pc as usize] as u16) << 8 | self.memory[self.pc as usize + 1] as u16;
        self.pc += 2;

        self.decode(opcode);
    }

    fn decode(&mut self, opcode: u16){

        //all this is in the format 0xXYZW, where X is the instruction and YZW is additional data

        
        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => {
                    self.graphics = [0u8; 2048];
                } //clear the screen
                0x00EE => {

                } //return from a subroutine
                _ => {

                } // some bs check wikipedia
            }

            0x1000 => {
                self.pc = (opcode & 0x0FFF) as i16;
            } //move the pointer to YZW

            0x2000 => {
                //TO BE IMPLEMENTED
            } //calls the subroutine at YZW

            0x3000 => {
                let x: u16= opcode & 0x0F00 >> 8;
                let vx: u8= self.registers[x as usize];

                let xx: u16 = opcode & 0x00FF;

                if vx == xx as u8 {
                    self.pc += 2;
                }
            }

            0x4000 => {
                let x: u16= opcode & 0x0F00 >> 8;
                let vx: u8= self.registers[x as usize];

                let xx: u16 = opcode & 0x00FF;

                if vx != xx as u8 {
                    self.pc += 2;
                }
            }

            _ => {
                panic!("unknown opcode: {:#X}", opcode);
            }
        }
    }
}