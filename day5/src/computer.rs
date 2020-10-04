use std::convert::TryFrom;
use std::io::{self, Write};

pub type Memory<'a> = &'a mut [i64];

pub struct Computer<'a> {
    pc: usize,
    memory: Memory<'a>,
}

enum Instruction {
    Binary(Binary),
    Input(Input),
    Output(Output),
    Halt,
    Jump(Jump),
    Comparison(Comparison),
}

enum ComparisonKind {
    LessThan,
    Equals,
}

struct Comparison {
    kind: ComparisonKind,
    target: usize,
    op1: usize,
    op2: usize,
}

impl Comparison {
    fn execute(self, computer: &mut Computer) {
        let result = match self.kind {
            ComparisonKind::Equals => computer.memory[self.op1] == computer.memory[self.op2],
            ComparisonKind::LessThan => computer.memory[self.op1] < computer.memory[self.op2],
        };
        computer.memory[self.target] = if result { 1 } else { 0 };
        computer.pc += 4;
    }
}

enum JumpCondition {
    True,
    False,
}
struct Jump {
    kind: JumpCondition,
    cond: usize,
    to: usize,
}

impl Jump {
    fn execute(self, computer: &mut Computer) {
        let condition = match self.kind {
            JumpCondition::True => computer.memory[self.cond] != 0,
            JumpCondition::False => computer.memory[self.cond] == 0,
        };
        computer.pc = if condition {
            computer.memory[self.to] as usize
        } else {
            computer.pc + 3
        };
    }
}

enum BinaryKind {
    Multiply,
    Plus,
}

struct Binary {
    kind: BinaryKind,
    target: usize,
    op1: usize,
    op2: usize,
}

impl Binary {
    fn execute(self, computer: &mut Computer) {
        let res = match self.kind {
            BinaryKind::Multiply => computer.memory[self.op1] * computer.memory[self.op2],
            BinaryKind::Plus => computer.memory[self.op1] + computer.memory[self.op2],
        };
        computer.memory[self.target] = res;
        computer.pc += 4;
    }
}

struct Input {
    target: usize,
}

impl Input {
    fn execute(self, computer: &mut Computer) {
        print!(">");
        io::stdout().flush().unwrap();
        let mut buf = String::new();
        computer.memory[self.target] = match std::io::stdin().read_line(&mut buf) {
            Ok(_) => buf.trim().parse::<i64>().expect("Input was no number"),
            Err(_) => unimplemented!("Input was not possible"),
        };
        computer.pc += 2;
    }
}

struct Output {
    target: usize,
}

impl Output {
    fn execute(self, computer: &mut Computer) {
        println!("{}", computer.memory[self.target]);
        computer.pc += 2;
    }
}

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
}

impl TryFrom<i64> for Mode {
    type Error = &'static str;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            _ => Err("Unknown instruction mode"),
        }
    }
}

fn split_instruction(instr: i64) -> (i64, Mode, Mode, Mode) {
    let inst = instr % 100;
    let op1_mode = Mode::try_from((instr / 100) % 10).unwrap();
    let op2_mode = Mode::try_from((instr / 1000) % 10).unwrap();
    let op3_mode = Mode::try_from((instr / 10000) % 10).unwrap();
    (inst, op1_mode, op2_mode, op3_mode)
}

impl<'a> Computer<'a> {
    pub fn new(memory: &mut [i64]) -> Computer {
        Computer { pc: 0, memory }
    }

    pub fn run(&mut self) -> i64 {
        while !self.finished() {
            self.step();
        }
        self.result()
    }

    fn parameter_index(&self, offset: usize, mode: Mode) -> usize {
        match mode {
            Mode::Position => self.memory[self.pc + offset] as usize,
            Mode::Immediate => self.pc + offset,
        }
    }

    fn parse_instruction(&self) -> Instruction {
        let split = split_instruction(self.memory[self.pc]);

        match split {
            (1, mode1, mode2, mode3) | (2, mode1, mode2, mode3) => {
                return Instruction::Binary(Binary {
                    kind: if split.0 == 1 {
                        BinaryKind::Plus
                    } else {
                        BinaryKind::Multiply
                    },
                    target: self.parameter_index(3, mode3),
                    op1: self.parameter_index(1, mode1),
                    op2: self.parameter_index(2, mode2),
                })
            }
            (3, mode1, _, _) => {
                return Instruction::Input(Input {
                    target: self.parameter_index(1, mode1),
                })
            }
            (4, mode1, _, _) => {
                return Instruction::Output(Output {
                    target: self.parameter_index(1, mode1),
                })
            }
            (5, mode1, mode2, _) | (6, mode1, mode2, _) => {
                return Instruction::Jump(Jump {
                    kind: if split.0 == 5 {
                        JumpCondition::True
                    } else {
                        JumpCondition::False
                    },
                    cond: self.parameter_index(1, mode1),
                    to: self.parameter_index(2, mode2),
                })
            }
            (7, mode1, mode2, mode3) | (8, mode1, mode2, mode3) => {
                return Instruction::Comparison(Comparison {
                    kind: if split.0 == 7 {
                        ComparisonKind::LessThan
                    } else {
                        ComparisonKind::Equals
                    },
                    target: self.parameter_index(3, mode3),
                    op1: self.parameter_index(1, mode1),
                    op2: self.parameter_index(2, mode2),
                })
            }
            (99, _, _, _) => return Instruction::Halt,
            a => {
                print!("{:?}", a);
                unreachable!("Bug in intcode program");
            }
        }
    }

    fn step(&mut self) {
        let ins = self.parse_instruction();
        //TODO: this is shit does rust have no abstraction to not require this here?
        match ins {
            Instruction::Halt => (),
            Instruction::Binary(b) => b.execute(self),
            Instruction::Output(o) => o.execute(self),
            Instruction::Input(i) => i.execute(self),
            Instruction::Jump(j) => j.execute(self),
            Instruction::Comparison(c) => c.execute(self),
        }
    }

    fn finished(&self) -> bool {
        self.memory[self.pc] == 99
    }

    fn result(&self) -> i64 {
        self.memory[0]
    }
}
