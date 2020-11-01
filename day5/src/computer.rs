use std::convert::TryFrom;
use std::io::{self, Write};

pub type Memory<'a> = &'a mut [i64];

pub struct Computer<'a> {
    pc: usize,
    memory: Memory<'a>,
}

enum BinaryKind {
    Multiply,
    Plus,
}
enum Instruction {
    Binary {
        kind: BinaryKind,
        target: usize,
        op1: usize,
        op2: usize,
    },
    Input {
        target: usize,
    },
    Output {
        target: usize,
    },
    Halt,
    Jump {
        kind: JumpCondition,
        cond: usize,
        to: usize,
    },
    Comparison {
        kind: ComparisonKind,
        target: usize,
        op1: usize,
        op2: usize,
    },
}

enum JumpCondition {
    True,
    False,
}
enum ComparisonKind {
    LessThan,
    Equals,
}

#[derive(Debug)]
enum Mode {
    Position,
    Immediate,
}

impl TryFrom<i64> for Mode {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            x => Err(format!("Unknown instruction mode {}", x)),
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

    fn execute_instruction(&mut self, ins: Instruction) {
        let new_pc = match ins {
            Instruction::Comparison {
                kind,
                target,
                op1,
                op2,
            } => {
                let result = match kind {
                    ComparisonKind::Equals => self.memory[op1] == self.memory[op2],
                    ComparisonKind::LessThan => self.memory[op1] < self.memory[op2],
                };
                self.memory[target] = if result { 1 } else { 0 };
                self.pc + 4
            }
            Instruction::Jump { kind, cond, to } => {
                let condition = match kind {
                    JumpCondition::True => self.memory[cond] != 0,
                    JumpCondition::False => self.memory[cond] == 0,
                };
                if condition {
                    self.memory[to] as usize
                } else {
                    self.pc + 3
                }
            }
            Instruction::Binary {
                kind,
                target,
                op1,
                op2,
            } => {
                let res = match kind {
                    BinaryKind::Multiply => self.memory[op1] * self.memory[op2],
                    BinaryKind::Plus => self.memory[op1] + self.memory[op2],
                };
                self.memory[target] = res;
                self.pc + 4
            }
            Instruction::Halt => self.pc,
            Instruction::Input { target } => {
                print!(">");
                io::stdout().flush().unwrap();
                let mut buf = String::new();
                self.memory[target] = match std::io::stdin().read_line(&mut buf) {
                    Ok(_) => buf.trim().parse::<i64>().expect("Input was no number"),
                    Err(_) => unimplemented!("Input was not possible"),
                };
                self.pc + 2
            }
            Instruction::Output { target } => {
                println!("{}", self.memory[target]);
                self.pc + 2
            }
        };
        self.pc = new_pc;
    }

    fn parse_instruction(&self) -> Instruction {
        let split = split_instruction(self.memory[self.pc]);

        match split {
            (1, mode1, mode2, mode3) | (2, mode1, mode2, mode3) => {
                return Instruction::Binary {
                    kind: if split.0 == 1 {
                        BinaryKind::Plus
                    } else {
                        BinaryKind::Multiply
                    },
                    target: self.parameter_index(3, mode3),
                    op1: self.parameter_index(1, mode1),
                    op2: self.parameter_index(2, mode2),
                }
            }
            (3, mode1, _, _) => {
                return Instruction::Input {
                    target: self.parameter_index(1, mode1),
                }
            }
            (4, mode1, _, _) => {
                return Instruction::Output {
                    target: self.parameter_index(1, mode1),
                }
            }
            (5, mode1, mode2, _) | (6, mode1, mode2, _) => {
                return Instruction::Jump {
                    kind: if split.0 == 5 {
                        JumpCondition::True
                    } else {
                        JumpCondition::False
                    },
                    cond: self.parameter_index(1, mode1),
                    to: self.parameter_index(2, mode2),
                }
            }
            (7, mode1, mode2, mode3) | (8, mode1, mode2, mode3) => {
                return Instruction::Comparison {
                    kind: if split.0 == 7 {
                        ComparisonKind::LessThan
                    } else {
                        ComparisonKind::Equals
                    },
                    target: self.parameter_index(3, mode3),
                    op1: self.parameter_index(1, mode1),
                    op2: self.parameter_index(2, mode2),
                }
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
        self.execute_instruction(ins);
    }

    fn finished(&self) -> bool {
        self.memory[self.pc] == 99
    }

    fn result(&self) -> i64 {
        self.memory[0]
    }
}
