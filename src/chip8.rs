const STACK_SIZE: usize = 24;

struct Stack {
    stack: [u16; STACK_SIZE],
    stack_pointer: usize,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            stack: [0; STACK_SIZE],
            stack_pointer: 0,
        }
    }

    fn push(&mut self, val: u16) {
        // que pasa si llego al final????
        self.stack_pointer += 1;
        self.stack[self.stack_pointer] = val;
    }

    fn pop(&mut self) -> u16 {
        // que pasa si ya estaba vacío el stack???
        self.stack_pointer -= 1;
        self.stack[self.stack_pointer]
    }
}

pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT: usize = 32;

pub struct Screen {
    // monocromático? es bool==????
    screen: [[bool; SCREEN_WIDTH]; SCREEN_HEIGHT],
}

impl Screen {
    fn new() -> Screen {
        Screen {
            screen: [[false; SCREEN_WIDTH]; SCREEN_HEIGHT],
        }
    }

    fn set_pixel(&mut self, pos: (usize, usize), val: bool) {
        // validar x, y?
        self.screen[pos.1][pos.0] = val;
    }

    pub fn get_pixel(&self, pos:(usize, usize)) -> bool {
        self.screen[pos.1][pos.0]
    }
}

const MEMORY_SIZE: usize = 0x1000;
const REGISTERS: usize = 16;
const PROGRAM_COUNTER_START: u16 = 0x200;

const FONT: [u8; 80] = [
    0x60, 0xa0, 0xa0, 0xa0, 0xc0,
    0x40, 0xc0, 0x40, 0x40, 0xe0,
    0xc0, 0x20, 0x40, 0x80, 0xe0,
    0xc0, 0x20, 0x40, 0x20, 0xc0,
    0x20, 0xa0, 0xe0, 0x20, 0x20,
    0xe0, 0x80, 0xc0, 0x20, 0xc0,
    0x40, 0x80, 0xc0, 0xa0, 0x40,
    0xe0, 0x20, 0x60, 0x40, 0x40,
    0x40, 0xa0, 0x40, 0xa0, 0x40,
    0x40, 0xa0, 0x60, 0x20, 0x40,
    0x40, 0xa0, 0xe0, 0xa0, 0xa0,
    0xc0, 0xa0, 0xc0, 0xa0, 0xc0,
    0x60, 0x80, 0x80, 0x80, 0x60,
    0xc0, 0xa0, 0xa0, 0xa0, 0xc0,
    0xe0, 0x80, 0xc0, 0x80, 0xe0,
    0xe0, 0x80, 0xc0, 0x80, 0x80,
];

const FONT_POSITION: usize = 0x0;
const FILE_POSITION: usize = 0x200;

pub struct Chip8 {
    memory: [u8; MEMORY_SIZE],
    registers: [u8; REGISTERS],
    program_counter: u16,
    index: u16,
    stack: Stack,
    delay_timer: u8,
    sound_timer: u8,
    pub screen: Screen,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut c = Chip8 {
            memory: [0; MEMORY_SIZE],
            registers: [0; REGISTERS],
            program_counter: PROGRAM_COUNTER_START,
            index: 0,
            stack: Stack::new(),
            delay_timer: 0,
            sound_timer: 0,
            screen: Screen::new(), 
        };
        c.load_font();

        c
    }

    fn load_font(&mut self) {
        self.memory[FONT_POSITION..FONT.len()].clone_from_slice(&FONT);
    }


    pub fn load_rom(&mut self) -> Result<(), anyhow::Error> {
        // ver el tamaño del archivo???
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open("game.ch8")?;

        let mut data = vec![];
        file.read_to_end(&mut data)?;

        let length = FILE_POSITION + data.len();
        self.memory[FILE_POSITION..length].clone_from_slice(&data);

        println!("Loaded rom");

        Ok(())
    }

    pub fn step(&mut self) {
        //fetch
        let pc = self.program_counter as usize;
        let opcode: u16 = 
            ((self.memory[pc] as u16) << 8) | self.memory[pc + 1] as u16;
        self.program_counter += 2;

        //decode
        let nibbles = nibbles_from_u16(opcode);
        let address: u16 = opcode & 0xFFF;
        let byte_2: u8 = (opcode & 0xFF) as u8;

        //execute
        self.execute(nibbles, address, byte_2);
    }

    fn execute(&mut self, nibbles: [u8; 4], address: u16, byte_2: u8) {
        match nibbles {
            [0x0, 0x0, 0xE, 0x0] => {
                // clear
                self.clear();
            },
            [0x0, 0x0, 0xE, 0xE] => {
                // return
                self.program_counter = self.stack.pop();
            },
            [0x1, _, _, _] => {
                // jump
                self.program_counter = address;
            },
            [0x2, _, _, _] => {
                // call
                self.stack.push(self.program_counter);
                self.program_counter = address;
            },
            [0x3, X, _, _] => {
                // skip if equal
                if self.registers[X as usize] == byte_2 {
                    self.program_counter += 2;
                }
            },
            [0x4, X, _, _] => {
                // skip if not equal
                if self.registers[X as usize] != byte_2 {
                    self.program_counter += 2;
                }
            },
            [0x5, X, Y, 0x0] => {
                // skip if X == Y
                if self.registers[X as usize] == self.registers[Y as usize] {
                    self.program_counter += 2;
                }
            },
            [0x6, X, _, _] => {
                // set
                self.registers[X as usize] = byte_2;
            },
            [0x7, X, _, _] => {
                // add
                let X = X as usize;

                self.registers[X] = self.registers[X].wrapping_add(byte_2);
            },
            [0x8, X, Y, 0x0] => {
                // set
                self.registers[X as usize] = self.registers[Y as usize];
            },
            [0x8, X, Y, 0x1] => {
                // or
                self.registers[X as usize] |= self.registers[Y as usize];
            },
            [0x8, X, Y, 0x2] => {
                // and
                self.registers[X as usize] &= self.registers[Y as usize];
            },
            [0x8, X, Y, 0x3] => {
                // xor
                self.registers[X as usize] ^= self.registers[Y as usize];
            },
            [0x8, X, Y, 0x4] => {
                // cuando esto salga de nightly, usarlo acá: https://doc.rust-lang.org/std/primitive.u8.html#method.carrying_add
                // add
                let (X, Y) = (X as usize, Y as usize);

                let val_x = self.registers[X];
                let val_y = self.registers[Y];

                self.registers[X] = val_x.wrapping_add(val_y);
                self.registers[0xF] = val_x.checked_add(val_y).is_none().into();
            },
            [0x8, X, Y, 0x5] => {
                // mismo problema del carrying add
                // sub
                let (X, Y) = (X as usize, Y as usize);

                let val_x = self.registers[X];
                let val_y = self.registers[Y];

                self.registers[X] = val_x.wrapping_sub(val_y);
                self.registers[0xF] = val_x.checked_sub(val_y).is_some().into();
            },
            [0x8, X, _, 0x6] => {
                // shift right
                let X = X as usize;

                self.registers[0xF] = self.registers[X] & 0x1;
                self.registers[X] >>= 1;
            },
            [0x8, X, Y, 0x7] => {
                // Y - X
                let (X, Y) = (X as usize, Y as usize);

                let val_x = self.registers[X];
                let val_y = self.registers[Y];

                self.registers[X] = val_y.wrapping_sub(val_x);
                self.registers[0xF] = (val_y > val_x).into();
            },
            [0x8, X, _, 0xE] => {
                // shift left
                let X = X as usize;

                self.registers[0xF] = self.registers[X] >> 7;
                self.registers[X] <<= 1;
            },
            // 0x9
            [0xA, _, _, _] => {
                // set index
                self.index = address;
            },
            // 0xB, 0xC
            [0xD, X, Y, N] => {
                // draw
                let (X, Y) = (X as usize, Y as usize);
                self.draw(X, Y, N);

            }
            _ => {
                eprintln!("Unknown opcode {:x}, {:x}, {:x}, {:x}", nibbles[0], nibbles[1], nibbles[2], nibbles[3]);
            }

        }
    }

    fn clear(&mut self) {
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                self.screen.set_pixel((x, y), false);
            }
        }
    }

    fn draw(&mut self, x: usize, y: usize, n: u8) {
        self.registers[0xF] = 0;

        for i in 0..(n as usize) {
            let val = self.memory[self.index as usize + i]; // fila de 8 pixeles

            for j in 0..8 {
                let val = val >> (7 - j);
                if self.screen.get_pixel((x + j, y + i)) && val == 0 {
                    self.registers[0xF] = 1;
                }

                self.screen.set_pixel((x + j, y + i), val != 0);
            }
        }
    }

}

fn nibbles_from_u16(var: u16) -> [u8; 4] {
    [(var >> 12) as u8,
    (var >> 8 & 0xF) as u8,
    (var >> 4 & 0xF) as u8,
    (var & 0xF) as u8]
}
