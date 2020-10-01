#[derive(Copy, Clone)]
struct Number {
    parts: [u8; 6],
}

impl Number {
    fn new(number: u32) -> Option<Number> {
        let mut number = number;
        if number < 100000 || number > 999999 {
            return None;
        }
        let mut parts = [0u8; 6];
        for part in &mut parts[..] {
            *part = (number % 10) as u8;
            number /= 10;
        }
        for idx in (1..parts.len()).rev() {
            if parts[idx - 1] < parts[idx] {
                parts[idx - 1] = parts[idx]
            }
        }
        Some(Number { parts })
    }

    fn to_number(&self) -> u32 {
        self.parts
            .iter()
            .enumerate()
            .map(|(i, val)| 10u32.pow(i as u32) * *val as u32)
            .sum()
    }

    fn has_doubles(&self) -> bool {
        self.parts[..].windows(2).any(|wind| wind[0] == wind[1])
    }

    //Have seen a hashmap used here pretty cleverly. This code is definitely not readable and should be reworked.
    fn has_doubles_no_triples(&self) -> bool {
        let mut double = false;
        let mut longer = false;
        let mut last = self.parts[0];
        for &p in self.parts[1..].into_iter() {
            if p == last && !double {
                double = true
            } else if p == last && double {
                longer = true;
            } else if p != last && double && !longer {
                return true;
            } else if p != last {
                double = false;
                longer = false;
            }
            last = p;
        }
        if double && !longer {
            return true;
        }
        return false
    }
}

//assumes acc is already in our password format!
fn next(mut acc: [u8; 6]) -> Option<[u8; 6]> {
    let mut idx = 0;
    while idx != acc.len() {
        if acc[idx] < 9 {
            acc[idx] += 1;
            for prev in 0..idx {
                acc[prev] = acc[idx];
            }
            return Some(acc);
        }
        idx += 1;
    }
    return None;
}

impl Iterator for Number {
    type Item = Self;
    fn next(&mut self) -> Option<Self> {
        if self.to_number() == 0 {
            return None
        }

        let res = Number{parts: self.parts };
        if let Some(new_part) = next(self.parts) {
            self.parts = new_part;
        } else {
            self.parts = [0; 6]; //poison value to know there was no more number.
        }
        Some(res)
    }
}

fn main() {
    let test_number = Number::new(254032).expect("should be valid");
    let mut count_p1 = 0;
    let mut count_p2 = 0;
    for num in test_number.into_iter() {
        if num.to_number() > 789860 { break; }
        if num.has_doubles() {
            count_p1 += 1;
        }
        if num.has_doubles_no_triples() {
            count_p2 += 1;
        }
    }
    println!("{}", count_p1);
    println!("{}", count_p2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn number_test() {
        let number = 123456;
        let num = Number::new(number).expect("should work");
        assert_eq!(number, num.to_number());
    }

    #[test]
    fn number_next() {
        let mut num = Number::new(123456).expect("should work");
        num.next();
        assert_eq!(123457, num.next().unwrap().to_number());

        let mut num2 = Number::new(899999).expect("should work");
        num2.next();
        assert_eq!(999999, num2.next().unwrap().to_number())
    }

    #[test]
    fn number_doubles() {
        let num = Number::new(111111).expect("should work");
        assert!(num.has_doubles());
    }

    #[test]
    fn number_test2() {
        let num = Number::new(254032).expect("should work");
        assert_eq!(num.to_number(), 255555);
        assert!(num.has_doubles());
    }

    #[test]
    fn triples_test() {
        let num = Number::new(112233).expect("should work");
        assert!(num.has_doubles_no_triples());
        let num2 = Number::new(123444).expect("should work");
        assert!(!num2.has_doubles_no_triples());
        let num3 = Number::new(111122).expect("should work");
        assert!(num3.has_doubles_no_triples());
        let num4 = Number::new(111222).expect("should work");
        assert!(!num4.has_doubles_no_triples());
    }
}
