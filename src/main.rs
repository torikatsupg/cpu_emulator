#![allow(dead_code)]

struct CPU {
    registers: [u8; 16], // この2つのレジスタだけで加算できる
    position_in_memory: usize, // プログラムカウンタ
    memory: [u8; 0x1000], // 1000000000000 = 2^12 = 4096 = 4kb
    stack: [u16; 16],
    stack_pointer: usize,
}

impl CPU {
    fn init() -> CPU {
        CPU {
            registers: [0; 16],
            memory: [0; 4096],
            position_in_memory: 0,
            stack: [0; 16],
            stack_pointer: 0,
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
    
    /// 関数呼び出しは
    /// 1. プログラムカウンタをスタックに格納
    /// 2. スタックポインタをインクリメント
    /// 3. 現在のメモリ位置にCALLの飛び先メモリアドレスをセットする
    /// 関数リターンは
    /// 1. スタックポインタをデクリメント
    /// 2. スタックに格納した戻り先アドレスを取り出す
    /// 3. プログラムカウンタに取り出したアドレスをセットする
    fn call(&mut self, addr: u16) {
        let sp = self.stack_pointer;
        let stack = &mut self.stack;

        if sp > stack.len() {
            panic!("Stack overflow!")
        }

        stack[sp] = self.position_in_memory as u16;
        self.stack_pointer += 1;
        self.position_in_memory = addr as usize;
    }

    fn ret(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        let call_addr = self.stack[self.stack_pointer];
        self.position_in_memory = call_addr as usize;
    }

    fn run(&mut self) {
        loop {
            let opecode = self.read_opecode();
            self.position_in_memory += 2;

            let nnn = opecode & 0x0FFF;

            let c = ((opecode & 0xF000) >> 12) as u8;
            let x = ((opecode & 0x0F00) >>  8) as u8;
            let y = ((opecode & 0x00F0) >>  4) as u8;
            let d = ((opecode & 0x000F) >>  0) as u8;

            match (c, x, y, d) {
                (0, 0, 0, 0) => { return; }, // 0x0000を終了コードとする
                (0, 0, 0xE, 0xE) => self.ret(),
                (0x2, _, _, _) => self.call(nnn),
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

    let mem = &mut cpu.memory;

    mem[0x000] = 0x21; mem[0x001] = 0x00;
    mem[0x002] = 0x21; mem[0x003] = 0x00;
    mem[0x004] = 0x00; mem[0x005] = 0x00;

    mem[0x100] = 0x80; mem[0x101] = 0x14;
    mem[0x102] = 0x80; mem[0x103] = 0x14;
    mem[0x104] = 0x00; mem[0x105] = 0xEE;

    cpu.run();

    assert_eq!(cpu.registers[0], 45);
    println!("5 + (10 * 2) + (10 * 2) = {}", cpu.registers[0]);
}
