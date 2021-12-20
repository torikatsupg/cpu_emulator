struct CPU {
    registers: [u8; 16], // この2つのレジスタだけで加算できる
    position_in_memory: usize, // プログラムカウンタ
    memory: [u8; 0x1000], // 1000000000000 = 2^12 = 4096 = 4kb
}

impl CPU {
    fn init() -> CPU {
        CPU {
            registers: [0; 16],
            memory: [0; 4096],
            position_in_memory: 0,
        }
    }

    fn read_opecode(&self) -> u16 {
        // 2byte一気に読みたい
        let p = self.position_in_memory;
        let op_byte1 = self.memory[p] as u16;
        let op_byte2 = self.memory[p + 1] as u16;

        // 1byteずつ取ってきて、バイトを連結する
        // xxxxxxxx_00000000(op_byte1) | 00000000_yyyyyyyy(op_byte2)
        // -> xxxxxxxx_yyyyyyyy
        op_byte1 << 8 | op_byte2
    }

    fn run(&mut self) {
        loop {
            let opecode = self.read_opecode();
            self.position_in_memory += 2;

            let c = ((opecode & 0xF000) >> 12) as u8;
            let x = ((opecode & 0x0F00) >>  8) as u8;
            let y = ((opecode & 0x00F0) >>  4) as u8;
            let d = ((opecode & 0x000F) >>  0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => { return; }, // 0x0000を終了コードとする
                (0x8, _, _, 0x4) => self.add_xy(x, y),
                _ => todo!("opecode {:04x}", opecode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let arg1 = self.registers[x as usize];
        let arg2 = self.registers[y as usize];

        let (val, overflow) = arg1.overflowing_add(arg2); // (u8, bool) を返す
        self.registers[x as usize] = val;

        // 最後のレジスタ(0xF)をキャリーフラグとして扱い、オーバーフローの検知に使う
        if overflow {
            self.registers[0xF] = 1;
        } else {
            self.registers[0xF] = 0;
        }
    }
}

fn main() {
    let mut cpu = CPU::init();

    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 10;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;
    mem[0] = 0x80; mem[1] = 0x14; // 8014
    mem[2] = 0x80; mem[3] = 0x24; // 8014
    mem[4] = 0x80; mem[5] = 0x34; // 8014

    cpu.run();

    assert_eq!(cpu.registers[0], 35);

    println!("5 + 10 = {}", cpu.registers[0]);
}
