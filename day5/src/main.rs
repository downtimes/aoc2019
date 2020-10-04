mod computer;

use crate::computer::Computer;

fn parse_input(input: &str) -> Vec<i64> {
    input.split(',').map(|s| s.parse::<i64>().unwrap()).collect()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Input file not found.");
    let mut memory = parse_input(&input);
    let mut computer = Computer::new(&mut memory);
    computer.run();
}
