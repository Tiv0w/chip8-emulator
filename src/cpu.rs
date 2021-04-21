use crate::bus::Bus;
use crate::utils;
use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Cpu {
    delay: u8,
    i: u16,
    pc: u16,
    sound: u8,
    sp: u16,
    stack: [u16; 16],
    v: [u8; 16],
    rng: ThreadRng,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            delay: 0,
            i: 0x0A,
            pc: 0x50,
            sound: 0,
            sp: 0,
            stack: [0; 16],
            v: [0; 16],
            rng: rand::thread_rng(),
        }
    }

    pub fn run(&mut self, bus: &mut Bus) {
        let opcode = self.get_opcode_at_address(bus, self.pc);
        self.execute_opcode(bus, opcode);
        self.next_instruction();
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
                println!("Clearscreen op");
                bus.display.clear();
            }
            [0x0, 0x0, 0xE, 0xE] => {
                println!("Return from chip8 subroutine");
            }
            [0x0, a, b, c] => {
                println!("Perfect {} {} {}", a, b, c);
            }
            [0x1, ..] => {
                println!("Jump to {}", opcode & 0x0FFF);
                self.pc = opcode & 0x0FFF;
            }
            [0x2, ..] => {
                println!("Call subroutine at {}", opcode & 0x0FFF);
                let subroutine_opcode = self.get_opcode_at_address(bus, opcode & 0x0FFF);
                self.execute_opcode(bus, subroutine_opcode);
            }
            [0x3, x, ..] => {
                println!("Skip next instruction if equal to {}", opcode & 0x00FF);
                let vx = self.v[x as usize];
                if vx == (opcode & 0x00FF) as u8 {
                    self.skip_next_instruction();
                }
            }
            [0x4, x, ..] => {
                println!("Skip next instruction if not equal to {}", opcode & 0x00FF);
                let vx = self.v[x as usize];
                if vx != (opcode & 0x00FF) as u8 {
                    self.skip_next_instruction();
                }
            }
            [0x5, x, y, 0x0] => {
                println!("Skip next instruction if V{} == V{}", x, y);
                let vx = self.v[x as usize];
                let vy = self.v[y as usize];
                if vx == vy {
                    self.skip_next_instruction();
                }
            }
            [0x5, ..] => {
                println!("Illegal instruction");
            }
            [0x6, x, ..] => {
                println!("Assign V{} = {}", x, opcode & 0x00FF);
                self.v[x as usize] = (opcode & 0x00FF) as u8;
            }
            [0x7, x, ..] => {
                println!("Add {} to V{}", opcode & 0x00FF, x);
                let vx: u16 = self.v[x as usize] as u16;
                let add_result: u16 = (opcode & 0x00FF) + vx;
                self.v[x as usize] = (add_result & 0x00FF) as u8;
            }
            [0x8, x, y, 0x0] => {
                println!("Assign V{} to V{}", y, x);
                self.v[x as usize] = self.v[y as usize];
            }
            [0x8, x, y, 0x1] => {
                println!("Set V{} to (V{} OR V{})", x, x, y);
                self.v[x as usize] = self.v[x as usize] | self.v[y as usize];
            }
            [0x8, x, y, 0x2] => {
                println!("Set V{} to (V{} AND V{})", x, x, y);
                self.v[x as usize] = self.v[x as usize] & self.v[y as usize];
            }
            [0x8, x, y, 0x3] => {
                println!("Set V{} to (V{} XOR V{})", x, x, y);
                self.v[x as usize] = self.v[x as usize] ^ self.v[y as usize];
            }
            [0x8, x, y, 0x4] => {
                println!("Add V{} to V{}", y, x);
                let vx: u16 = self.v[x as usize] as u16;
                let vy: u16 = self.v[y as usize] as u16;
                let add_result: u16 = vx + vy;
                self.v[x as usize] = (add_result & 0x00FF) as u8;
                self.v[0xF] = if (add_result >> 8) > 0 { 1 } else { 0 };
            }
            [0x8, x, y, 0x5] => {
                println!("Subtract V{} to V{}", y, x);
                let vx: i8 = self.v[x as usize] as i8;
                let vy: i8 = self.v[y as usize] as i8;
                let sub_result: i8 = vx - vy;
                self.v[x as usize] = sub_result as u8;
                self.v[0xF] = if sub_result < 0 { 0 } else { 1 };
            }
            [0x8, x, _, 0x6] => {
                println!("Shifts V{} by 1, stores last bit in VF", x);
                let vx = self.v[x as usize];
                self.v[0xF] = vx & 0x1;
                self.v[x as usize] = vx >> 1;
            }
            [0x8, x, y, 0x7] => {
                println!("Stores in V{} V{} - V{}", x, y, x);
                let vx: i8 = self.v[x as usize] as i8;
                let vy: i8 = self.v[y as usize] as i8;
                let sub_result: i8 = vy - vx;
                self.v[x as usize] = sub_result as u8;
                self.v[0xF] = if sub_result < 0 { 0 } else { 1 };
            }
            [0x8, x, _, 0xE] => {
                println!("Shifts left V{} by 1, stores first bit in VF", x);
                let vx = self.v[x as usize];
                self.v[0xF] = vx & 0x80;
                self.v[x as usize] = vx << 1;
            }
            [0x8, ..] => {
                println!("Illegal instruction");
            }
            [0x9, x, y, 0x0] => {
                println!("Skip next instruction if V{} != V{}", x, y);
                let vx = self.v[x as usize];
                let vy = self.v[y as usize];
                if vx != vy {
                    self.skip_next_instruction();
                }
            }
            [0x9, ..] => {
                println!("Illegal instruction");
            }
            [0xA, ..] => {
                println!("Set I to address {}", opcode & 0x0FFF);
                self.i = opcode & 0x0FFF;
            }
            [0xB, ..] => {
                println!("Jump to address {} + V0", opcode & 0x0FFF);
                self.pc = opcode & 0x0FFF + self.v[0] as u16;
            }
            [0xC, x, ..] => {
                println!("Set V{} to Rand AND {}", x, opcode & 0x00FF);
                let constant = (opcode & 0x00FF) as u8;
                let rand: u8 = self.rng.gen();
                self.v[x as usize] = rand & constant;
            }
            [0xD, x, y, n] => {
                println!("Draw {} {} {}", x, y, n);
                self.draw(bus, x, y, n);
            }
            [0xE, x, 0x9, 0xE] => {
                println!("KeyOp {}", x);
                // TODO: implement
            }
            [0xE, x, 0xA, 0x1] => {
                println!("KeyOp {}", x);
                // TODO: implement
            }
            [0xF, x, 0x0, 0x7] => {
                println!("Set V{} to value of delay timer", x);
                self.v[x as usize] = self.delay;
            }
            [0xF, x, 0x0, 0xA] => {
                println!("KeyOp {}", x);
                // TODO: implement
            }
            [0xF, x, 0x1, 0x5] => {
                println!("Set delay timer to V{}", x);
                self.delay = self.v[x as usize];
            }
            [0xF, x, 0x1, 0x8] => {
                println!("Set sound timer to V{}", x);
                self.sound = self.v[x as usize];
            }
            [0xF, x, 0x1, 0xE] => {
                println!("Add V{} to I", x);
                let vx: u8 = self.v[x as usize];
                self.i += vx as u16;
            }
            [0xF, x, 0x2, 0x9] => {
                println!("Set I to sprite address for char in V{}", x);
                // TODO: implement
            }
            [0xF, x, 0x3, 0x3] => {
                println!("BCD V{}", x);
                // TODO: implement
            }
            [0xF, x, 0x5, 0x5] => {
                println!("Stores V0 to V{} in memory", x);
                let x = x as usize;
                for idx in 0usize..=x {
                    let v = self.v[idx];
                    bus.memory.write_byte(self.i as usize + idx, v);
                }
            }
            [0xF, x, 0x6, 0x5] => {
                println!("Fills V0 to V{} from memory", x);
                let x = x as usize;
                for idx in 0usize..=x {
                    let byte = bus.memory.read_byte(self.i as usize + idx);
                    self.v[idx] = byte;
                }
            }
            [a, b, c, d] => {
                println!(
                    "Not implemented for now or illegal: {} {} {} {}",
                    a, b, c, d
                );
            }
        }
    }

    fn set_vf(&mut self, value: u8) {
        self.v[0xF] = value;
    }

    fn skip_next_instruction(&mut self) {
        self.pc += 2;
    }

    fn next_instruction(&mut self) {
        self.pc += 2;
    }

    fn draw(&mut self, bus: &mut Bus, x: u8, y: u8, n: u8) {
        let vx: usize = self.v[x as usize] as usize;
        let vy: usize = self.v[y as usize] as usize;

        let mut sprite_vec: Vec<u8> = Vec::new();
        for j in 0..=n {
            let address = self.i + j as u16;
            sprite_vec.push(bus.memory.read_byte(address as usize));
        }

        let collision = bus.display.draw((vx, vy), &sprite_vec);
        self.set_vf(collision as u8);
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
