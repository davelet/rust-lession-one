use crate::interface::Minimum;

#[derive(Clone)]
pub struct BigInteger {
    data: Vec<u8>,
}

impl BigInteger {
    pub(crate) fn print(&self) {
        print!("stored len: {}, value: ", self.data.len());
        let mut  index = 0;
        while index < self.data.len() {
            index += 1;
            print!("{}", self.data[self.data.len() - index])
        }
        println!()
    }
}

impl BigInteger {
    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.data = data;
    }
}

impl BigInteger {
    pub fn new(x: u8) -> Self {
        // if x == 0 {
        //     BigInteger { data: vec![] }
        // } else {
            BigInteger { data: vec![x] }
        // }
    }

    pub fn default() -> Self {
        BigInteger {data: vec![0]}
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
        BigInteger {data}
    }
}

impl Copy for BigInteger {
    fn clone(&self) -> Self {
        BigInteger {data: self.data.clone()}
    }
}

impl Minimum for BigInteger {
    fn compare(&self, s: Self) -> Self {
        todo!()
    }
}