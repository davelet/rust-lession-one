mod definition;

use std::{io, thread};
use std::io::BufRead;
use std::time::Duration;

fn main() {
    let v = read_vec();
    let duration = Duration::from_secs(1);
    let mut max = 0;
    for i in v {
        // thread::sleep(duration);
        if i > max { max = i }
        // println!("{}", i)
    }
    println!("max {}", max)
    println!("{}", a::a)
}
enum a{a}

fn read_vec() -> Vec<i32> {
    let mut vec = Vec::<i32>::new();
    // let mut vec: Vec<i32> = Vec::new();

    let io = io::stdin();
    for line in io.lock().lines() {
        let line = line.unwrap();
        match line.trim().parse::<i32>() {
            Ok(num) => {vec.push(num)}
            Err(err) => {println!("err: {}", err)}
        }
    }

    vec
}


// #
// Exercise 03.1: The goal is to write a generic version of SomethingOrNothing::print.
// To this end, define a trait Print that provides (simple) generic printing, and implement that trait for i32.
// Then define SomethingOrNothing::print2 to use that trait, and change main above to use the new generic print2 function.
// I will again provide a skeleton for this solution.
// It also shows how to attach bounds to generic implementations (just compare it to the impl block from the previous exercise).
// You can read this as “For all types T satisfying the Print trait, I provide an implementation for SomethingOrNothing<T>”.
//
// Notice that I called the function on SomethingOrNothing print2 to disambiguate from the print defined previously.
//
// Hint: There is a macro print! for printing without appending a newline.
//
// pub trait Print {
//     /* Add things here */
// }
// impl Print for i32 {
//     /* Add things here */
// }
// impl<T: Print> SomethingOrNothing<T> {
//     fn print2(self) {
//         unimplemented!()
//     }
// }
// #
// Exercise 03.2: Building on exercise 02.2, implement all the things you need on f32 to make your program work with floating-point numbers.