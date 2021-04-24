use super::bus::Bus;
use super::utils;
use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Cpu {
    delay: u8,
    i: u16,
    pc: u16,
    sound: u8,
    sp: u16,
    stack: Vec<u16>,
    v: [u8; 16],
    rng: ThreadRng,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            delay: 0,
            i: 0,
            pc: 0x200,
            sound: 0,
            sp: 0,
            stack: Vec::with_capacity(16),
            v: [0; 16],
            rng: rand::thread_rng(),
        }
    }

    pub fn run(&mut self, bus: &mut Bus) {
        let opcode = self.get_opcode_at_address(bus, self.pc);
        println!("{:#X}", opcode);
        self.execute_opcode(bus, opcode);
        if self.delay > 0 {
            self.delay -= 1;
        }
        if self.sound > 0 {
            self.sound -= 1;
        }
    }

    fn get_opcode_at_address(&self, bus: &Bus, address: u16) -> u16 {
        let add: usize = address as usize;
        let opcode_high: u16 = ((bus.memory.read_byte(add)) as u16) << 8;
        let opcode_low: u16 = (bus.memory.read_byte(add + 1)) as u16;
        opcode_high | opcode_low
    }

    pub fn execute_opcode(&mut self, bus: &mut Bus, opcode: u16) {
        let hex_digits: [u8; 4] = utils::get_hex_digits(opcode);
        match hex_digits {
            [0x0, 0x0, 0xE, 0x0] => {
                println!("Clearscreen");
                bus.display.clear();
                self.next_instruction();
            }
            [0x0, 0x0, 0xE, 0xE] => {
                println!("Return from subroutine");
                match self.stack.pop() {
                    Some(pc) => self.pc = pc,
                    None => println!("Error: trying to return from main program"),
                }
                self.next_instruction();
            }
            [0x1, ..] => {
                println!("Jump to {:#X}", opcode & 0x0FFF);
                self.pc = opcode & 0x0FFF;
            }
            [0x2, ..] => {
                println!("Call subroutine at {:#X}", opcode & 0x0FFF);
                self.stack.push(self.pc);
                self.pc = opcode & 0x0FFF;
            }
            [0x3, x, ..] => {
                println!("Skip next instruction if equal to {:#X}", opcode & 0x00FF);
                let vx = self.v[x as usize];
                if vx == (opcode & 0x00FF) as u8 {
                    self.next_instruction();
                }
                self.next_instruction();
            }
            [0x4, x, ..] => {
                println!(
                    "Skip next instruction if not equal to {:#X}",
                    opcode & 0x00FF
                );
                let vx = self.v[x as usize];
                if vx != (opcode & 0x00FF) as u8 {
                    self.next_instruction();
                }
                self.next_instruction();
            }
            [0x5, x, y, 0x0] => {
                println!("Skip next instruction if V{:#X} == V{:#X}", x, y);
                let vx = self.v[x as usize];
                let vy = self.v[y as usize];
                if vx == vy {
                    self.next_instruction();
                }
                self.next_instruction();
            }
            [0x5, ..] => {
                println!("Illegal instruction");
            }
            [0x6, x, ..] => {
                println!("Assign V{:#X} = {:#X}", x, opcode & 0x00FF);
                self.v[x as usize] = (opcode & 0x00FF) as u8;
                self.next_instruction();
            }
            [0x7, x, ..] => {
                println!("Add {:#X} to V{:#X}", opcode & 0x00FF, x);
                let vx: u16 = self.v[x as usize] as u16;
                let add_result: u16 = (opcode & 0x00FF) + vx;
                self.v[x as usize] = (add_result & 0x00FF) as u8;
                self.next_instruction();
            }
            [0x8, x, y, 0x0] => {
                println!("Assign V{:#X} to V{:#X}", y, x);
                self.v[x as usize] = self.v[y as usize];
                self.next_instruction();
            }
            [0x8, x, y, 0x1] => {
                println!("Set V{:#X} to (V{:#X} OR V{:#X})", x, x, y);
                self.v[x as usize] = self.v[x as usize] | self.v[y as usize];
                self.next_instruction();
            }
            [0x8, x, y, 0x2] => {
                println!("Set V{:#X} to (V{:#X} AND V{:#X})", x, x, y);
                self.v[x as usize] = self.v[x as usize] & self.v[y as usize];
                self.next_instruction();
            }
            [0x8, x, y, 0x3] => {
                println!("Set V{:#X} to (V{:#X} XOR V{:#X})", x, x, y);
                self.v[x as usize] = self.v[x as usize] ^ self.v[y as usize];
                self.next_instruction();
            }
            [0x8, x, y, 0x4] => {
                println!("Add V{:#X} to V{:#X}", y, x);
                let vx: u16 = self.v[x as usize] as u16;
                let vy: u16 = self.v[y as usize] as u16;
                let add_result: u16 = vx + vy;
                self.v[x as usize] = (add_result & 0x00FF) as u8;
                self.v[0xF] = if (add_result >> 8) > 0 { 1 } else { 0 };
                self.next_instruction();
            }
            [0x8, x, y, 0x5] => {
                println!("Subtract V{:#X} to V{:#X}", y, x);
                let vx: i8 = self.v[x as usize] as i8;
                let vy: i8 = self.v[y as usize] as i8;
                let sub_result: i8 = vx - vy;
                self.v[x as usize] = sub_result as u8;
                self.v[0xF] = if sub_result < 0 { 0 } else { 1 };
                self.next_instruction();
            }
            [0x8, x, _, 0x6] => {
                println!("Shifts V{:#X} by 1, stores last bit in VF", x);
                let vx = self.v[x as usize];
                self.v[0xF] = vx & 0x1;
                self.v[x as usize] = vx >> 1;
                self.next_instruction();
            }
            [0x8, x, y, 0x7] => {
                println!("Stores in V{:#X} V{:#X} - V{:#X}", x, y, x);
                let vx: i8 = self.v[x as usize] as i8;
                let vy: i8 = self.v[y as usize] as i8;
                let sub_result: i8 = vy - vx;
                self.v[x as usize] = sub_result as u8;
                self.v[0xF] = if sub_result < 0 { 0 } else { 1 };
                self.next_instruction();
            }
            [0x8, x, _, 0xE] => {
                println!("Shifts left V{:#X} by 1, stores first bit in VF", x);
                let vx = self.v[x as usize];
                self.v[0xF] = vx & 0x80;
                self.v[x as usize] = vx << 1;
                self.next_instruction();
            }
            [0x8, ..] => {
                println!("Illegal instruction");
            }
            [0x9, x, y, 0x0] => {
                println!("Skip next instruction if V{:#X} != V{:#X}", x, y);
                let vx = self.v[x as usize];
                let vy = self.v[y as usize];
                if vx != vy {
                    self.next_instruction();
                }
                self.next_instruction();
            }
            [0x9, ..] => {
                println!("Illegal instruction");
            }
            [0xA, ..] => {
                println!("Set I to address {:#X}", opcode & 0x0FFF);
                self.i = opcode & 0x0FFF;
                self.next_instruction();
            }
            [0xB, ..] => {
                println!("Jump to address {:#X} + V0", opcode & 0x0FFF);
                self.pc = opcode & 0x0FFF + self.v[0] as u16;
            }
            [0xC, x, ..] => {
                println!("Set V{:#X} to Rand AND {:#X}", x, opcode & 0x00FF);
                let constant = (opcode & 0x00FF) as u8;
                let rand: u8 = self.rng.gen();
                self.v[x as usize] = rand & constant;
                self.next_instruction();
            }
            [0xD, x, y, n] => {
                println!("Draw at (V{:#X}, V{:#X}) {:#X} lines", x, y, n);
                self.draw(bus, x, y, n);
                self.next_instruction();
            }
            [0xE, x, 0x9, 0xE] => {
                println!("KeyOp skip if V{:#X} key pressed", x);
                let vx: u8 = self.v[x as usize];
                if bus.input.get_current_key() == Some(vx) {
                    self.next_instruction();
                }
                self.next_instruction();
            }
            [0xE, x, 0xA, 0x1] => {
                println!("KeyOp skip if V{:#X} key not pressed", x);
                let vx: u8 = self.v[x as usize];
                if bus.input.get_current_key() != Some(vx) {
                    self.next_instruction();
                }
                self.next_instruction();
            }
            [0xF, x, 0x0, 0x7] => {
                println!("Set V{:#X} to value of delay timer", x);
                self.v[x as usize] = self.delay;
                self.next_instruction();
            }
            [0xF, x, 0x0, 0xA] => {
                println!("KeyOp store key in V{:#X}", x);
                match bus.input.get_current_key() {
                    Some(input) => {
                        self.v[x as usize] = input;
                        self.next_instruction();
                    }
                    None => {}
                }
            }
            [0xF, x, 0x1, 0x5] => {
                println!("Set delay timer to V{:#X}", x);
                self.delay = self.v[x as usize];
                self.next_instruction();
            }
            [0xF, x, 0x1, 0x8] => {
                println!("Set sound timer to V{:#X}", x);
                self.sound = self.v[x as usize];
                self.next_instruction();
            }
            [0xF, x, 0x1, 0xE] => {
                println!("Add V{:#X} to I", x);
                let vx: u8 = self.v[x as usize];
                self.i += vx as u16;
                self.next_instruction();
            }
            [0xF, x, 0x2, 0x9] => {
                println!("Set I to sprite address for char in V{:#X}", x);
                let vx = self.v[x as usize];
                self.i = (vx * 5) as u16;
                self.next_instruction();
            }
            [0xF, x, 0x3, 0x3] => {
                println!("BCD V{:#X}", x);
                let vx: u8 = self.v[x as usize];
                let bcd_digits = self.get_bcd(vx);
                for j in 0usize..3usize {
                    let address: usize = self.i as usize + j;
                    bus.memory.write_byte(address, bcd_digits[j]);
                }
                self.next_instruction();
            }
            [0xF, x, 0x5, 0x5] => {
                println!("Stores V0 to V{:#X} in memory", x);
                let x = x as usize;
                for idx in 0usize..=x {
                    let v = self.v[idx];
                    bus.memory.write_byte(self.i as usize + idx, v);
                }
                self.next_instruction();
            }
            [0xF, x, 0x6, 0x5] => {
                println!("Fills V0 to V{:#X} from memory", x);
                let x = x as usize;
                for idx in 0usize..=x {
                    let byte = bus.memory.read_byte(self.i as usize + idx);
                    self.v[idx] = byte;
                }
                self.next_instruction();
            }
            [a, b, c, d] => {
                println!(
                    "Not implemented for now or illegal: {:#X} {:#X} {:#X} {:#X}",
                    a, b, c, d
                );
            }
        }
    }

    fn set_vf(&mut self, value: u8) {
        self.v[0xF] = value;
    }

    fn next_instruction(&mut self) {
        self.pc += 2;
    }

    fn draw(&mut self, bus: &mut Bus, x: u8, y: u8, n: u8) {
        let vx: usize = self.v[x as usize] as usize;
        let vy: usize = self.v[y as usize] as usize;

        let mut sprite_vec: Vec<u8> = Vec::new();
        for j in 0..n {
            let address = self.i + j as u16;
            sprite_vec.push(bus.memory.read_byte(address as usize));
        }

        let collision = bus.display.draw((vx, vy), &sprite_vec);
        self.set_vf(collision as u8);
    }

    fn get_bcd(&mut self, x: u8) -> [u8; 3] {
        let mut bcd_digits: [u8; 3] = [0; 3];
        bcd_digits[2] = x % 10;
        bcd_digits[1] = x / 10 % 10;
        bcd_digits[0] = x / 100;

        bcd_digits
    }
}

// part of debugging
impl std::fmt::Debug for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Cpu {{\n\tdelay: {},\n\ti: {},\n\tsound: {},\n\tpc: {},\n\tsp: {},\n\tstack: {:?},\n\tv: {:?}\n}}",
            self.delay,
            self.i,
            self.sound,
            self.pc,
            self.sp,
            self.stack,
            self.v
        )
    }
}

// part of debugging
impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Cpu {{\n\ti: {:#X},\n\tpc: {:#X},\n\tv: {:?}\n}}",
            self.i, self.pc, self.v
        )
    }
}
