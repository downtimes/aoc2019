use itertools::Itertools;

type Memory<'a> = &'a mut [i32];
const STEP_SIZE: usize = 4;

struct Computer<'a> {
    pc: usize,
    memory: Memory<'a>,
}

impl<'a> Computer<'a> {
    fn step(&mut self) {
        //TODO: clean up this implementation.
        match self.memory[self.pc] {
            1 => self.memory[self.memory[self.pc + 3] as usize] = self.memory[self.memory[self.pc + 1] as usize] + self.memory[self.memory[self.pc + 2] as usize],
            2 => self.memory[self.memory[self.pc + 3] as usize] = self.memory[self.memory[self.pc + 1] as usize] * self.memory[self.memory[self.pc + 2] as usize],
            99 => return,
            _ => unreachable!(),
        }
        self.pc += STEP_SIZE;
    }

    fn finished(&self) -> bool {
        self.memory[self.pc] == 99
    }
    
    fn result(&self) -> i32 {
        self.memory[0]
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Input file not found.");
    let parsed_input = input.split(",").filter_map(|s| s.parse::<i32>().ok()).collect::<Vec<_>>();
    let mut memory1 = parsed_input.clone(); 
    //Fix up input for part1
    memory1[1] = 12;
    memory1[2] = 2;
    let mut computer = Computer{ pc: 0, memory: &mut memory1};
    while !computer.finished() {
        computer.step();
    }
    println!("{}", computer.result());

    let mut result2 = 0;
    for (noun, verb) in (0..=99).tuple_combinations() {
        let mut memory = parsed_input.clone();
        memory[1] = noun;
        memory[2] = verb;

        let mut computer = Computer{ pc: 0, memory: &mut memory};
        while !computer.finished() {
            computer.step();
        }
        if computer.result() == 19690720 {
            result2 = 100 * noun + verb;
            break;
        }
    }
    println!("{}", result2);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let mut memory = vec![1, 0, 0, 0, 99];
        let mut computer = Computer{ pc: 0, memory: &mut memory };
        computer.step();
        assert!(computer.finished());
        computer.step();
        assert!(computer.finished());
        assert_eq!(computer.result(), 2);
    }

    #[test]
    fn test2() {
        let mut memory = vec![2, 3, 0, 3, 99];
        let mut computer = Computer{ pc: 0, memory: &mut memory };
        computer.step();
        assert!(computer.finished());
        computer.step();
        assert!(computer.finished());
        assert_eq!(computer.result(), 2);
    }

    #[test]
    fn test3() {
        let mut memory = vec![2, 4, 4, 5, 99, 0];
        let mut computer = Computer{ pc: 0, memory: &mut memory };
        while !computer.finished() {
            computer.step();
        }
        assert_eq!(computer.result(), 2);
    }

    #[test]
    fn test4() {
        let mut memory = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut computer = Computer{ pc: 0, memory: &mut memory };
        while !computer.finished() {
            computer.step();
        }
        assert_eq!(computer.result(), 30);
    }
}
