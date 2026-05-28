use rand::Rng;

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

    key: [bool; 16],

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
            key: [false; 16],
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
                self.stack[self.sp as usize] = self.pc;
                self.sp +=1 ;
                self.pc = (opcode & 0x0FFF) as i16;
            } //calls the subroutine at YZW

            0x3000 => {
                let x: u16= (opcode & 0x0F00) >> 8;
                let vx: u8= self.registers[x as usize];

                let xx: u16 = opcode & 0x00FF;

                if vx == xx as u8 {
                    self.pc += 2;
                }
            } //skips the next instruction if Vx = ZW

            0x4000 => {
                let x: u16= (opcode & 0x0F00) >> 8;
                let vx: u8= self.registers[x as usize];

                let xx: u16 = opcode & 0x00FF;

                if vx != xx as u8 {
                    self.pc += 2;
                }
            } //skips the next instruction if Vx != ZW

            0x5000 => {
                let x: u16= (opcode & 0x0F00) >> 8;
                let y: u16= (opcode & 0x00F0) >> 4;

                let vx: u8 = self.registers[x as usize];
                let vy: u8 = self.registers[y as usize];

                if vx != vy {
                    self.pc += 2;
                }
            } //skips the next instruction if Vx = Vy

            0x6000 => {
                let x: u16= (opcode & 0x0F00) >> 8;
                self.registers[x as usize] = (opcode & 0x00FF) as u8;

            } //sets the value of Vx

            0x7000 => {
                let x: u16= (opcode & 0x0F00) >> 8;
                self.registers[x as usize] = self.registers[x as usize].wrapping_add((opcode & 0x00FF) as u8);
            } //adds ZW to Vx

            0x8000 => {
                let x: u16= (opcode & 0x0F00) >> 8;
                let y: u16= (opcode & 0x00F0) >> 4;

                match opcode & 0x000F {

                    0x0000 => {
                        self.registers[x as usize] = self.registers[y as usize];
                    } //sets Vx to Vy

                    0x0001 => {
                        self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize];
                    } //sets Vx to Vx OR Vy

                    0x0002 => {
                        self.registers[x as usize] = self.registers[x as usize] & self.registers[y as usize];
                    }

                    0x0003 => {
                        self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize];
                    }

                    0x0004 => {
                        let result: u16 = self.registers[x as usize] as u16 + self.registers[y as usize] as u16;
                        self.registers[0xF] = if result > 255 { 1 } else { 0 };
                        self.registers[x as usize] = result as u8;
                    }

                    0x0005 => {
                        let result: u16 = self.registers[x as usize] as u16 - self.registers[y as usize] as u16;
                        self.registers[0xF] = if self.registers[x as usize] > self.registers[y as usize] { 1 } else { 0 };
                        self.registers[x as usize] = result as u8;
                    }
                    0x0006 => {
                        self.registers[0xF] = self.registers[x as usize] & 0x1;
                        self.registers[x as usize] >>= 1;
                    }

                    0x0007 => {
                        let result: u16 = self.registers[y as usize] as u16 - self.registers[x as usize] as u16;
                        self.registers[0xF] = if self.registers[y as usize] > self.registers[x as usize] { 1 } else { 0 };
                        self.registers[x as usize] = result as u8;
                    }

                    0x000E => {
                        self.registers[0xF] = if self.registers[x as usize] & 0x80 == 0x80 {1} else {0};
                        self.registers[x as usize] <<= 1;
                    }

                    _ => panic!("unknown opcode: {:#X}", opcode)
                }
            }

            0x9000 => {
                let x: u16= (opcode & 0x0F00) >> 8;
                let y: u16= (opcode & 0x00F0) >> 4;

                if self.registers[x as usize] != self.registers[y as usize] {
                    self.pc += 2;
                }
            }

            0xA000 => {
                self.I = (opcode & 0x0FFF) as i16;
            }

            0xB000 => {
                let x: u16= (opcode & 0x0F00) >> 8;

                self.pc = (opcode & 0x0FFF) as i16 + self.registers[x as usize] as i16;
            }

            0xC000 => {
                let rand: u8 = rand::thread_rng().gen_range(0..=255);
                let x: u16= (opcode & 0x0F00) >> 8;
                let xx: u16 = opcode & 0x00FF;

                self.registers[x as usize] = rand & xx as u8;

            }

            0xD000 => {
                let x: u16= (opcode & 0x0F00) >> 8;
                let y: u16= (opcode & 0x00F0) >> 4;
                let n: u8 = (opcode & 0x000F) as u8;

                let vx: u8 = self.registers[x as usize];
                let vy: u8 = self.registers[y as usize];
                self.registers[0xF] = 0;

                for i in 0..n {
                    let sprite: u8 = self.memory[self.I as usize + i as usize];
                    for j in 0..8 {
                        let pos: usize = ((vy as u16 + i as u16) % 32 * 64 + (vx as u16+ j as u16) % 64) as usize;

                        if sprite & (0x80 >> j) != 0 {
                            if self.graphics[pos] == 1 {
                                self.registers[0xF] = 1;
                            }
                            self.graphics[pos] ^= 1;
                        }
                        
                    }
                    
                }


            } //draws stuff, I is a pointer to the start of a sprite

            0xE000 => {
                let x: u16= (opcode & 0x0F00) >> 8;
                let vx = self.registers[x as usize];

                
                match opcode & 0x00FF {
                    0x009E => {
                        
                        if self.key[vx as usize] {
                            self.pc += 2;
                        }
                    }

                    0x00A1 => {
                        if !self.key[vx as usize] {
                            self.pc += 2;
                        }
                    }

                    _ => panic!("unknown opcode: {:#X}", opcode)
                }
            }

            0x000F => {
                let x: u16= (opcode & 0x0F00) >> 8;
                let vx = self.registers[x as usize];

                match opcode & 0x00FF {
                    
                    0x0007 => {
                        self.registers[x as usize] = self.delay_timer;
                    }

                    0x000A => {
                        //read the wikipedia, basically stops instruction until the next key event, timers keep counting down
                    }

                    0x0015 => {
                        self.delay_timer = vx;
                    }

                    0x0018 => {
                        self.sound_timer = vx;
                    }

                    0x001E => {
                        self.I += vx as i16;
                    }

                    0x0029 => {
                        //sets I to the location of the sprite for the character in vx. characters are represented by a 4x5 font
                    }

                    0x0033 => {
                        //read the wikipedia page
                    }

                    0x0055 => {
                        for i in 0..x+1 {
                            self.memory[self.I as usize + i as usize] = self.registers[i as usize]
                        }
                    } //stores from v0 to vx starting at address I, leaving I unchanged

                    0x0065 => {
                        for i in 0..x+1 {
                            self.registers[i as usize]= self.memory[self.I as usize + i as usize];
                        }
                    }

                    _ => panic!("unknown opcode: {:#X}", opcode)
                }
            }
            _ => {
                panic!("unknown opcode: {:#X}", opcode);
            }
        }
    }
}