mod sums {
    use std::fmt;

    #[derive(PartialEq, PartialOrd)]
    pub struct NonincSum {
        sum: u32,
        terms: Vec<u32>,
    }

    impl NonincSum {
        pub fn new(v: Vec<u32>) -> NonincSum {
            // Let's do all the checks first:
            // Need at least two values.
            assert!(v.len() > 1);

            // All positive:
            assert!(v.iter().all(|&x| x > 0));

            // Vector is non-increasing:
            for idx in 1..v.len() {
                assert!(v[idx - 1] >= v[idx]);
            }

            let s = v.iter().sum();
            NonincSum { sum: s, terms: v }
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
            #[derive(Clone)]
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
                        if self.terms[idx] == (*last).num {
                            (*last).times += 1
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

            let mut msg: String = "< ".to_owned();
            for rn in compressed {
                if rn.times > 2 {
                    msg += format!("{}({}) ", rn.times, rn.num).as_str();
                } else if rn.times == 2 {
                    msg += format!("{} {} ", rn.num, rn.num).as_str();
                } else if rn.times == 1 {
                    msg += format!("{} ", rn.num).as_str();
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
    // This correctly fails:
    // let x = sums::NonincSum::new([0].to_vec());

    // This correctly fails:
    // let x = sums::NonincSum::new([0, 0].to_vec());

    // This correctly fails:
    //let x = sums::NonincSum::new([1, 2, 3].to_vec());

    let u = sums::NonincSum::new([3, 2, 1].to_vec());
    let v = sums::NonincSum::new([3, 2, 1].to_vec());
    assert!(u == v);
    let x = sums::NonincSum::new([5, 3, 1].to_vec());
    if x > v {
        println!("PartialOrd is working!");
    }
    //let u = sums::NonincSum::new([2, 2, 2].to_vec());
    println!("{}", u);


    let a = sums::NonincSum::new([2, 1, 1].to_vec());
    let b = sums::NonincSum::new([1, 1, 1, 1].to_vec());
    if a < b {
        println!("{} is less than {}", a, b);
    } else {
        println!("{} is not less than {}", a, b);
    }

    if b < x {
        println!("{} is less than {}", b, x);
    } else {
        println!("{} is not less than {}", b, x);
    }

    let c = sums::NonincSum::new([6, 2, 1].to_vec());
    if x < c {
        println!("{} is less than {}", x, c);
    } else {
        println!("{} is not less than {}", x, c);
    }
}
