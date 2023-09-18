mod sums {
    use std::fmt;

    #[derive(Debug, PartialEq, PartialOrd)]
    pub struct NonincSum {
        sum: u32,
        terms: Vec<u32>,
    }

    impl NonincSum {
        pub fn new(v: Vec<u32>) -> Result<NonincSum, &'static str> {
            // FIXME: Do this without `mut`
            let any_increase = || -> bool {
                let mut result = false;
                for idx in 1..v.len() {
                    if v[idx - 1] < v[idx] {
                        result = true;
                        break;
                    }
                }
                result
            };

            if v.len() < 2 {
                Err("Need at lest two elements")
            } else if v.iter().any(|&x| x <= 0) {
                Err("All elements must be positive")
            } else if any_increase() {
                Err("Element sequence must be non-increasing")
            } else {
                let s = v.iter().sum();
                Ok(NonincSum { sum: s, terms: v })
            }
        }

        // This method attempts to create a new sum by decrementing the
        // term at `dec` and incrementing at `inc`. If it's still a valid
        // sum, return it.
        pub fn generate(&self, dec: usize, inc: usize) -> Option<NonincSum> {
            assert!(dec < inc);
            let mut v = self.terms.clone();
            assert!(dec < v.len());
            assert!(inc < v.len());

            v[dec] -= 1;
            v[inc] += 1;
            match NonincSum::new(v) {
                Ok(ns) => Some(ns),
                Err(_) => None,
            }
        }
    }

    // Now the question is how to I define operations on a
    // NonincSum? Really this is only intersting in the context of generating
    // unique non-incrementing sums. I suppose I should check the identity operation
    // for starters. Derived traits are quite powerful.
    // Really the first thing to do is use a brute force approach to get the answers
    // for small N before trying bigger N.

    impl fmt::Display for NonincSum {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            #[derive(Clone, PartialEq)]
            struct RepeatedNum {
                num: u32,
                times: u32,
            }

            let mut compressed = [RepeatedNum {
                num: self.terms[0],
                times: 1,
            }]
            .to_vec();

            for idx in 1..self.terms.len() {
                match compressed.last_mut() {
                    Some(last) => {
                        if self.terms[idx] == last.num {
                            last.times += 1
                        } else {
                            compressed.push(RepeatedNum {
                                num: self.terms[idx],
                                times: 1,
                            })
                        }
                    }
                    None => panic!("`compressed` should not be empty!"),
                }
            }

            let mut msg: String = "<".to_owned();
            for rn in &compressed {
                let comma = match compressed.first() {
                    Some(val) => {
                        if rn == val {
                            ""
                        } else {
                            ", "
                        }
                    }
                    _ => "",
                };

                if rn.times > 2 {
                    msg += format!("{}{}({})", comma, rn.times, rn.num).as_str();
                } else if rn.times == 2 {
                    msg += format!("{}{}, {}", comma, rn.num, rn.num).as_str();
                } else if rn.times == 1 {
                    msg += format!("{}{}", comma, rn.num).as_str();
                } else {
                    panic!("Term should not occur zero times!");
                }
            }
            msg += ">";
            write!(f, "{}", msg)
        }
    }
}

fn main() {
    // TASK: Generate exhaustive unique sums for small numbers.
    // Could start with 2.
}

#[cfg(test)]
mod tests {
    use crate::sums::NonincSum;

    fn new_should_fail(v: Vec<u32>, desc: &'static str) {
        match NonincSum::new(v) {
            Ok(_) => panic!("{} should fail", desc),
            Err(_) => (),
        }
    }

    fn expect_display(v: Vec<u32>, disp: &'static str) {
        let sum = NonincSum::new(v).expect("valid args");
        assert_eq!(format!("{}", sum), disp);
    }

    fn expect_generate(u: Vec<u32>, i: usize, k: usize, v: Vec<u32>) {
        let sum = NonincSum::new(u).expect("valid args");
        let g = sum.generate(i, k).expect("valid indices");
        let e = NonincSum::new(v).expect("valid args");
        assert_eq!(g, e);
    }

    fn expect_generate_nothing(u: Vec<u32>, i: usize, k: usize) {
        let sum = NonincSum::new(u).expect("valid args");
        match sum.generate(i, k) {
            Some(_) => panic!(
                "Generating from {} at index {} and {} should yield nothing",
                sum, i, k
            ),
            None => (),
        }
    }

    #[test]
    fn empty_fails() {
        new_should_fail([].to_vec(), "empty");
    }

    #[test]
    fn single_element_fails() {
        new_should_fail([1].to_vec(), "single element");
    }

    #[test]
    fn zero_fails() {
        new_should_fail([1, 0].to_vec(), "zero");
    }

    #[test]
    fn increase_fails() {
        new_should_fail([1, 2, 3].to_vec(), "increasing elements");
    }

    #[test]
    fn two_element_display() {
        expect_display([1, 1].to_vec(), "<1, 1>");
    }

    #[test]
    fn three_same_display() {
        expect_display([1, 1, 1].to_vec(), "<3(1)>");
    }

    #[test]
    fn complex_display() {
        expect_display([5, 5, 5, 4, 4, 2, 2, 2, 2].to_vec(), "<3(5), 4, 4, 4(2)>");
    }

    #[test]
    fn generate_basic() {
        let v = [7, 3, 1].to_vec();
        expect_generate(v.clone(), 0, 1, [6, 4, 1].to_vec());
        expect_generate(v.clone(), 1, 2, [7, 2, 2].to_vec());
        expect_generate(v.clone(), 0, 2, [6, 3, 2].to_vec());
    }

    #[test]
    fn generate_nothing() {
        expect_generate_nothing([1, 1].to_vec(), 0, 1);
        expect_generate_nothing([4, 3, 2].to_vec(), 0, 1);
    }
}
