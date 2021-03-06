const SIZE: usize = 6;

struct Number {
    parts: [u8; SIZE],
}

impl Number {
    //TODO: We could implement the try from trait instead of this function too...
    fn new(number: u32) -> Option<Number> {
        let mut number = number;
        if number < 100000 || number > 999999 {
            return None;
        }
        let mut parts = [0u8; SIZE];
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

    fn has_doubles(&self) -> bool {
        self.parts[..].windows(2).any(|wind| wind[0] == wind[1])
    }

    //Have seen a hashmap used here pretty cleverly. This code is definitely not readable and should be reworked.
    fn has_doubles_no_triples(&self) -> bool {
        let mut follows = 0;
        let mut last_num = self.parts[0];
        for &curr in self.parts[1..].into_iter() {
            if curr == last_num {
                follows += 1;
            } else {
                if follows == 1 {
                    return true;
                }
                follows = 0;
            }
            last_num = curr;
        }
        //check for last 2 numbers equality
        if follows == 1 {
            return true;
        }
        false
    }

    fn to_u32(&self) -> u32 {
        self.parts
            .iter()
            .enumerate()
            .map(|(i, val)| 10u32.pow(i as u32) * *val as u32)
            .sum()
    }

    fn next(&self) -> Option<Self> {
        let mut idx = 0;
        let mut new_parts = self.parts;
        while idx != SIZE {
            if new_parts[idx] < 10 {
                new_parts[idx] += 1;
                for prev in 0..idx {
                    new_parts[prev] = new_parts[idx];
                }
                return Some(Number { parts: new_parts });
            }
            idx += 1;
        }
        return None;
    }
}

impl IntoIterator for Number {
    type Item = Self;
    type IntoIter = NumberIterator;

    fn into_iter(self) -> Self::IntoIter {
        NumberIterator {
            current: Some(self),
            limit: None,
        }
    }
}

struct NumberIterator {
    current: Option<Number>,
    limit: Option<u32>,
}

impl Iterator for NumberIterator {
    type Item = Number;
    fn next(&mut self) -> Option<Self::Item> {
        let mut res = self.current.take();
        if let Some(limit) = self.limit {
            res = res.filter(|num| num.to_u32() <= limit);
        }
        if let Some(ref num) = res {
            self.current = num.next();
        }
        res
    }
}

impl NumberIterator {
    fn set_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

fn main() {
    let test_number = Number::new(254032).expect("Not a number");
    let mut count_p1 = 0;
    let mut count_p2 = 0;
    for num in test_number.into_iter().set_limit(789860) {
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
        assert_eq!(number, num.to_u32());
    }

    #[test]
    fn number_next() {
        let num = Number::new(123456).expect("should work");
        let mut iter = num.into_iter();
        assert_eq!(123456, iter.next().unwrap().to_u32());

        let num2 = Number::new(899999).expect("should work");
        let mut iter2 = num2.into_iter();
        iter2.next();
        assert_eq!(999999, iter2.next().unwrap().to_u32())
    }

    #[test]
    fn limit_with_lower() {
        let num = Number::new(111111).expect("should work");
        let mut iter = num.into_iter().set_limit(0);
        assert!(iter.next().is_none())
    }

    #[test]
    fn number_doubles() {
        let num = Number::new(111111).expect("should work");
        assert!(num.has_doubles());
    }

    #[test]
    fn number_test2() {
        let num = Number::new(254032).expect("should work");
        assert_eq!(255555, num.to_u32());
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
