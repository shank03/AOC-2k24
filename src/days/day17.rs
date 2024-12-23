use super::Day;

#[derive(Clone)]
pub enum OpCode {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u64> for OpCode {
    fn from(value: u64) -> Self {
        let ops = [
            Self::Adv,
            Self::Bxl,
            Self::Bst,
            Self::Jnz,
            Self::Bxc,
            Self::Out,
            Self::Bdv,
            Self::Cdv,
        ];

        ops.get(value as usize).cloned().unwrap()
    }
}

#[derive(Clone)]
pub struct CPU {
    a: u64,
    b: u64,
    c: u64,
}
impl CPU {
    pub fn execute(&mut self, pc: &mut usize, op_code: OpCode, literal_op: u64) -> Option<u64> {
        let reg = [self.a, self.b, self.c];
        let operand = if literal_op >= 4 {
            reg[literal_op as usize - 4]
        } else {
            literal_op
        };

        match op_code {
            OpCode::Adv => self.a >>= operand,
            OpCode::Bxl => self.b ^= literal_op,
            OpCode::Bst => self.b = operand & 7,
            OpCode::Jnz => {
                if self.a != 0 {
                    *pc = operand as usize;
                }
            }
            OpCode::Bxc => self.b ^= self.c,
            OpCode::Out => return Some(operand & 7),
            OpCode::Bdv => self.b = self.a >> operand,
            OpCode::Cdv => self.c = self.a >> operand,
        };

        None
    }
}

pub struct Day17;

impl Day for Day17 {
    type Input = (CPU, Vec<u64>);
    fn parse_input(input: &str) -> Self::Input {
        let mut l = input.lines();
        let a = l.next().unwrap().split(": ").last().unwrap();
        let b = l.next().unwrap().split(": ").last().unwrap();
        let c = l.next().unwrap().split(": ").last().unwrap();

        let ins = l.last().unwrap().split(": ").last().unwrap();

        (
            CPU {
                a: a.parse().unwrap(),
                b: b.parse().unwrap(),
                c: c.parse().unwrap(),
            },
            ins.split(',').map(|i| i.parse().unwrap()).collect(),
        )
    }

    type OP1 = Vec<u64>;
    fn part_1((mut cpu, ins): Self::Input) -> Self::OP1 {
        let mut res = Vec::new();

        let mut pc = 0usize;
        while pc < ins.len() {
            let op_code = OpCode::from(ins[pc]);
            let operand = ins[pc + 1];

            pc += 2;
            if let Some(v) = cpu.execute(&mut pc, op_code, operand) {
                res.push(v);
            }
        }

        res
    }

    type OP2 = u64;
    fn part_2((cpu, ins): Self::Input) -> Self::OP2 {
        let exp = ins.clone();

        let mut res = 0;
        let mut done = false;

        while !done {
            for i in 0..0xFFFFFFFF {
                let mut cpu = CPU {
                    a: i + res,
                    b: cpu.b,
                    c: cpu.c,
                };

                let mut out = Vec::new();
                let mut pc = 0usize;
                while pc < ins.len() {
                    let op_code = OpCode::from(ins[pc]);
                    let operand = ins[pc + 1];

                    pc += 2;
                    if let Some(v) = cpu.execute(&mut pc, op_code, operand) {
                        out.push(v);
                    }
                }

                let mut all_good = true;
                for j in 0..out.len() {
                    if exp[exp.len() - j - 1] != out[out.len() - j - 1] {
                        all_good = false;
                        break;
                    }
                }

                if all_good {
                    res = (res + i) << 3;
                    if exp.len() == out.len() {
                        done = true;
                    }
                    break;
                }
            }
        }

        res >> 3
    }
}
