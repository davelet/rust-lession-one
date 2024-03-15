use std::cmp;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;
use crate::interface::Minimum;

// #[derive(Clone)]
pub struct BigInteger {
    data: Vec<u8>,
}

impl BigInteger {
    pub(crate) fn print(&self) {
        print!("stored len: {}, value: ", self.data.len());
        let mut index = 0;
        while index < self.data.len() {
            index += 1;
            print!("{}", self.data[self.data.len() - index])
        }
        println!()
    }
}

impl Display for BigInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut num = "".to_string();
        for index in 1..=self.data.len() {
            num.push_str(&(self.data[self.data.len() - index]).to_string());
        }
        write!(f, "{}", num)
    }
}

// impl BigInteger {
//     pub fn get_data(&self) -> &Vec<u8> {
//         &self.data
//     }
//
//     pub fn set_data(&mut self, data: Vec<u8>) {
//         self.data = data;
//     }
// }

impl BigInteger {
    pub fn new(x: u8) -> Self {
        // if x == 0 {
        //     BigInteger { data: vec![] }
        // } else {
        BigInteger { data: vec![x] }
        // }
    }

    pub fn default() -> Self {
        BigInteger { data: vec![0] }
    }

    pub fn test_invariant(&self) -> bool {
        if self.data.len() <= 1 {
            true
        } else {
            self.data[self.data.len() - 1] != 0
        }
    }

    pub fn from_vec(mut v: Vec<u8>) -> Self {
        let mut p = v.pop();
        while p.is_some() && p.unwrap() == 0 {
            p = v.pop()
        }
        if p.is_none() {
            return BigInteger::default();
        }
        v.push(p.unwrap());
        let mut data = vec![];
        for mut b in v {
            data.push(b % 10);
        }
        BigInteger { data }
    }
}


impl Minimum for BigInteger {
    // fn compare(&self, s:&Self) -> &Self {
    fn compare<'a>(&'a self, s: &'a Self) -> &'a Self {
        debug_assert!(self.test_invariant());
        debug_assert!(s.test_invariant());
        if self.data.len() < s.data.len() { self } else if self.data.len() > s.data.len() { s } else {
            let i = self.data.len();
            let mut j = i - 1;
            while j >= 0 {
                let this_digit = self.data[j];
                let other_digit = s.data[j];
                if this_digit > other_digit { return s; } else if this_digit < other_digit { return self; } else {
                    j = j - 1;
                }
            }
            s
        }
    }
}

impl<'a,'b> Add<&'a BigInteger> for &'b BigInteger {
    type Output = &'a BigInteger;

    fn add(self, rhs: &'a Self) -> Self::Output {
        let max_len = cmp::max(self.data.len(), rhs.data.len());
        let mut res_vec: Vec<u8> = Vec::with_capacity(max_len + 1);
        let mut carry_bit = 0_u8;
        for i in 0..max_len {
            let left = if i < self.data.len() { self.data[i] } else { 0 };
            let right = if i < rhs.data.len() { rhs.data[i] } else { 0 };
            let added = left + right + carry_bit;
            let bit = added % 10;
            carry_bit = if added >= 10 { 1 } else { 0 };
            res_vec.push(bit);
        }
        if carry_bit == 1 { res_vec.push(1) }
        BigInteger::from_vec(res_vec)
    }
}

#[test]
fn addit() {
    let a = BigInteger::from_vec(vec![7, 3, 1]);
    let b = BigInteger::from_vec(vec![3, 9, 2, 8]);
    let integer = a + b;
    println!("{} + {} = {}", a, b, integer);
}

impl PartialEq for BigInteger {
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl Debug for BigInteger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.data.fmt(f)
    }
}
