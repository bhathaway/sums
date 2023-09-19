mod sums {
    use std::fmt;

    #[derive(Debug, Hash, Eq, PartialEq, PartialOrd, Clone, Ord)]
    pub struct NonincSum {
        sum: usize,
        terms: Vec<usize>,
    }

    impl NonincSum {
        pub fn new(v: Vec<usize>) -> Result<NonincSum, &'static str> {
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

        // Appends `1` to the end of the terms, thereby increasing the sum
        // This method will always succeed
        pub fn append_at_end(&self) -> NonincSum {
            let NonincSum { sum: s, terms: v } = self;
            let one = vec![1];
            let new_terms = [&v[..], &one[..]].concat();
            NonincSum { sum: s + 1, terms: new_terms }
        }

        // Attempts to add `1` to the last term. If it violates the
        // constraint, returns None.
        pub fn inc_last(&self) -> Option<NonincSum> {
            let mut v = self.terms.clone();
            let len = v.len();
            assert!(len > 0);
            v[len - 1] += 1;
            //v.last().unwrap() += 1;
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
                num: usize,
                times: usize,
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

use std::collections::HashSet;
fn find_sums_restrict_terms(sum: usize, terms: usize) -> usize {
    assert!(terms <= sum);
    let mut unique_sums: HashSet<sums::NonincSum> = HashSet::new();
    let mut frontier: Vec<sums::NonincSum> = vec![];

    // First seed the frontier before starting.
    let mut seed_terms: Vec<usize> = vec![];
    seed_terms.push(sum - (terms - 1));
    for _ in 1..terms {
        seed_terms.push(1);
    }
    let seed = sums::NonincSum::new(seed_terms).expect("valid args");
    frontier.push(seed.clone());
    unique_sums.insert(seed);

    while frontier.len() > 0 {
        let mut next_frontier: Vec<sums::NonincSum> = vec![];
        for s in frontier {
            for i in 0..terms {
                for k in (i + 1)..terms {
                    match s.generate(i, k) {
                        Some(x) => {
                            if unique_sums.insert(x.clone()) {
                                next_frontier.push(x)
                            }
                        }
                        None => (),
                    }
                }
            }
        }

        frontier = next_frontier;
    }

    // Print the results
    for s in &unique_sums {
        println!("{}", s);
    }

    unique_sums.len()
}

fn find_sums(sum: usize) -> usize {
    let mut s = 0;
    for num_terms in 2..=sum {
        s += find_sums_restrict_terms(sum, num_terms)
    }

    s
}

// returns solutions for N from the solutions to N-1
fn next_layer(sum: usize, previous_sums: HashSet<sums::NonincSum>) -> HashSet<sums::NonincSum> {
    let mut unique_sums: HashSet<sums::NonincSum> = HashSet::new();
    unique_sums.insert(sums::NonincSum::new([sum - 1, 1].to_vec()).unwrap());

    for s in previous_sums {
        unique_sums.insert(s.append_at_end());
        match s.inc_last() {
            Some(ns) => unique_sums.insert(ns),
            None => false,
        };
    }

    unique_sums
}

fn find_sums_alt(sum: usize) -> usize {
    let mut working_set: HashSet<sums::NonincSum> = HashSet::new();

    for s in 2..=sum {
        working_set = next_layer(s, working_set);
    }

    let mut sorted: Vec<sums::NonincSum> = Vec::new();
    for ns in working_set {
        sorted.push(ns);
    }

    sorted.sort();
    for ns in &sorted {
        println!("{}", ns);
    }

    sorted.len()
}

fn main() {
    let args = std::env::args().nth(1);

    let sum = args.expect("no arg").parse::<usize>().ok().expect("not usize");
    println!("Brute force:");
    println!("Total: {}", find_sums(sum));
    println!("");
    println!("Dynamic programming:");
    println!("Total: {}", find_sums_alt(sum));
}

#[cfg(test)]
mod tests {
    use crate::sums::NonincSum;

    fn new_should_fail(v: Vec<usize>, desc: &'static str) {
        match NonincSum::new(v) {
            Ok(_) => panic!("{} should fail", desc),
            Err(_) => (),
        }
    }

    fn expect_display(v: Vec<usize>, disp: &'static str) {
        let sum = NonincSum::new(v).expect("valid args");
        assert_eq!(format!("{}", sum), disp);
    }

    fn expect_generate(u: Vec<usize>, i: usize, k: usize, v: Vec<usize>) {
        let sum = NonincSum::new(u).expect("valid args");
        let g = sum.generate(i, k).expect("valid indices");
        let e = NonincSum::new(v).expect("valid args");
        assert_eq!(g, e);
    }

    fn expect_generate_nothing(u: Vec<usize>, i: usize, k: usize) {
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

    #[test]
    fn append_at_end_basic() {
        let ns = NonincSum::new([6, 4, 1].to_vec()).unwrap();
        let next = ns.append_at_end();
        let expect = NonincSum::new([6, 4, 1, 1].to_vec()).unwrap();
        assert_eq!(next, expect);
    }

    #[test]
    fn inc_last_basic() {
        let ns = NonincSum::new([5, 5, 3].to_vec()).unwrap();
        let next = ns.inc_last().unwrap(); // Should have a result.
        let expect = NonincSum::new([5, 5, 4].to_vec()).unwrap();
        assert_eq!(next, expect);
    }
}
