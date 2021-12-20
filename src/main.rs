struct CPU {
    current_operation: u16, // CHIP-8のオペコードを全てu16
    registers: [u8; 2], // この2つのレジスタだけで加算できる
}

impl CPU {
    fn init() -> CPU {
        CPU {
            current_operation: 0, // 0が初期値
            registers: [0; 2],
        }
    }

    fn read_opecode(&self) -> u16 {
        self.current_operation
    }

    fn run(&mut self) {
        // loop {
            let opecode = self.read_opecode();

            let c = ((opecode & 0xF000) >> 12) as u8;
            let x = ((opecode & 0x0F00) >>  8) as u8;
            let y = ((opecode & 0x00F0) >>  4) as u8;
            let d = ((opecode & 0x000F) >>  0) as u8;

            match (c, x, y, d) {
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opecode {:04x}", opecode),
            }
        // }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
}

fn main() {
    let mut cpu = CPU::init();

    cpu.current_operation = 0x8014;
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;

    cpu.run();

    assert_eq!(cpu.registers[0], 15);

    println!("5 + 10 = {}", cpu.registers[0]);
}
