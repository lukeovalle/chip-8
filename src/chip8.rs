struct Stack {
    stack: [u16; 24],
    stack_pointer: usize,
}

impl Stack {
    fn new() -> Stack {
        Stack {
            stack: [0; 24],
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

pub struct Screen {
    // monocromático? es bool==????
    screen: [[bool; 64]; 32],
}

impl Screen {
    fn new() -> Screen {
        Screen {
            screen: [[false; 64]; 32],
        }
    }

    fn set_pixel(&mut self, pos: (usize, usize), val: bool) {
        // validar x, y?
        self.screen[pos.0][pos.1] = val;
    }

    pub fn get_pixel(&self, pos:(usize, usize)) -> bool {
        self.screen[pos.0][pos.1]
    }
}

pub struct Chip8 {
    memory: [u8; 0x1000],
    registers: [u8; 16],
    program_counter: usize,
    index: usize,
    stack: Stack,
    delay_timer: u8,
    sound_timer: u8,
    pub screen: Screen,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut c = Chip8 {
            memory: [0; 0x1000],
            registers: [0; 16],
            program_counter: 0x200,
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
        let font = [ 
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

        self.memory[0..80].clone_from_slice(&font);
    }


    pub fn load_rom(&mut self) -> Result<(), anyhow::Error> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open("game.ch8")?;

        let mut data = vec![];
        file.read_to_end(&mut data)?;
        let length = 0x200 + data.len();
        self.memory[0x200..length].clone_from_slice(&data);

        Ok(())
    }

    pub fn step(&muy self) {
        //fetch
        let opcode: u16 = (self.memory[self.program_counter] << 8) | self.memory[self.program_counter + 1];
        self.program_counter += 2;

        //decode
        let nibble_1: u8 = opcode >> 12;
        let nibble_2: u8 = (opcode >> 8) & 0xF;
        let nibble_3: u8 = (opcode >> 4) & 0xF;
        let nibble_4: u8 = opcode & 0xF;

        let address: u16 = opcode & 0xFFF;

        let byte_2: u8 = opcode & 0xFF;

        //execute
    }

    fn execute(&mut self, nibbles: [u8; 4]) {
        match nibbles.0 {
            0x0 => {
            },

        }
    }

}
