use std::io::prelude::*;
use std::thread;
use std::time::Duration;

use crate::closure::ClosureStorage;
use crate::definition::SomethingOrNothing::{self, *, Nothing, Something};
use crate::interface::Minimum;
use crate::r#struct::{BigInteger, PrintEachDigit};

mod interface;
mod r#struct;
mod definition;
mod closure;
mod concurrency;
mod file_handle;
mod linkedlist;

#[tokio::main]
async fn main() {

    // println!("1");
    // let mut vec = vec![18, 5, 7, 9, 27];
    // inc_vec(&vec, 3, 20);
    // let mut bigs: Vec<BigInteger> = vec![/*BigInteger::new(3),*/ BigInteger::from_vec(vec![5,2, 1]), BigInteger::from_vec(vec![4, 1,2]), BigInteger::default(), BigInteger::new(8)];
    // let raw_string_with_hash = r###"This is another raw string-------------# with a backslash: \ and a doubl"e quote:# ""###;
    // println!("{}",raw_string_with_hash);

    let vec1 = &mut vec![3];
    let f = first(vec1);
    println!("{}", match f {
        None => { "none".to_string()}
        Some(i) => { (*i).to_string() }
    });
    vec1.push(1);
    println!("l = {}", vec1.len());

    // println!("{}", raw_string_with_hash);
    // print_digits_in_big_int(&bigs[0], "she".to_string());
    // print_int_closure(&bigs[0], "she:".to_string());

    let mut  cs = ClosureStorage::default();
    // cs.register(Box::new(|a| println!("第一个回调 {}", a)));
    cs.call(100);
    // cs.register(Box::new(|a| println!("第2个回调 {}", a)));

    cs.call(200);
    {
        let mut i =0;
        cs.register(move |b| {
            i = i + 1;
            println!("第三个回调 {} {}", i, b)
        });
        cs.register(move |b| {
            i = i + 1;
            println!("第si个回调 {} {}", i, b)
        });
    }
    cs.call(300);
    cs.call(3000);
    cs.clone().call(400);
    cs.clone().call(4000);

    // redis().await;
    //
    // thread::sleep(Duration::from_secs(60));
}

fn print_digits_in_big_int(int: &BigInteger, pre: String) {
    let impl_action = PrintEachDigit { prefix: pre.clone() };
    int.act(impl_action);
}

fn print_int_closure(int: &BigInteger, pre: String) {
    let mut i = 0;
    int.act_fn(|d| {
        println!("{} : {}{}", i, pre, d);
        i = i + 1;
    });
}

fn inc_vec(vec: &Vec<i32>, off: i32, threshold: i32) -> Vec<i32> {
    // vec.iter().map(|d| d+off).filter(|d| *d>= threshold).for_each(|d| println!("{}", d));
    vec.iter().enumerate().for_each(|i| println!("{} {}", i.0, i.1));
    vec![]
    // vec.iter().map(|d| d+off).filter(|d| *d>= threshold).collect::<Vec<i32>>()
}

fn first<T>(vec: &Vec<T>) -> Option<&T> {
    if vec.len() > 0 { return Some(&vec[0]); }
    None
}

impl Minimum for i32 {
    fn compare<'a>(&'a self, s:&'a Self) -> &'a Self {
        if *self < *s { self } else { s }
    }
}

fn vec_min<T: Minimum>(vec: &Vec<T>) -> SomethingOrNothing<&T> {
    let mut min = Nothing;
    for e in vec {
        min = Something(match min {
            Something(i) => { e.compare(i) }
            Nothing => { e }
        })
    }

    min
}