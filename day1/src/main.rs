fn calculate_fuel(mass: u32) -> u64 {
    let temp = mass/3;
    if temp > 2 {
        (temp - 2) as u64
    } else {
        0
    }
}

//NOTE: schould work since the calculate_fuel function only ever reduces the mass
//      so it should converge to 0 eventually and is not an endless loop.
fn calculate_total_fuel(mass: u32) -> u64 {
    let mut next_mass = calculate_fuel(mass);
    let mut sum = 0;
    while next_mass > 0 {
        sum += next_mass;
        next_mass = calculate_fuel(next_mass as u32);
    }
    sum
}

fn calculate_sum_fuel(input: &[u32], func: fn(u32) -> u64) -> u64 {
    input.into_iter().map(|&val| func(val)).sum()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Input file not found!");
    let input_parsed = input.lines().filter_map(|s| s.parse::<u32>().ok()).collect::<Vec<u32>>();
    let res1 = calculate_sum_fuel(&input_parsed, calculate_fuel);
    let res2  = calculate_sum_fuel(&input_parsed, calculate_total_fuel);
    println!("{}", res1);
    println!("{}", res2);
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_calculation() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn test_calculated() {
        assert_eq!(calculate_total_fuel(14), 2);
        assert_eq!(calculate_total_fuel(1969), 966);
        assert_eq!(calculate_total_fuel(100756), 50346);
    }
}