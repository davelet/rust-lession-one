mod interface;
mod r#struct;
mod definition;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use crate::definition::SomethingOrNothing::{self, *, Nothing, Something};
use crate::interface::Minimum;
use crate::r#struct::BigInteger;

fn main() {
    /* Creating a Local TcpListener at Port 8477 */
    // const HOST : &str ="127.0.0.1";
    // const PORT : &str ="3000";
    // /* Concatenating Host address and Port to Create Final Endpoint */
    // let end_point : String = HOST.to_owned() + ":" +  PORT;
    // /*Creating TCP Listener at our end point */
    // let listener = TcpListener::bind(end_point).unwrap();
    // println!("Web server is listening at port {}",PORT);
    // /* Conneting to any incoming connections */
    // for stream in listener.incoming() {
    //     let _stream = stream.unwrap();
    //     // Call function to process any incomming connections
    //     handle_connection(_stream);
    // }

    // println!("1");
    // let mut vec = vec![18, 5, 7, 9, 27];
    // vec = vec![2];
    // let m = vec_min(&mut vec);
    // m.print();
    // vec_min(&mut vec).print();
    // vec.push(3);
    // println!("{}", vec.len());
    //
    // let mut bigs: Vec<BigInteger> = vec![/*BigInteger::new(3),*/ BigInteger::from_vec(vec![5,2, 1]), BigInteger::from_vec(vec![4, 1,2]), BigInteger::default(), BigInteger::new(8)];
    // vec_min(&mut bigs).print();

    let vec1 = &mut vec![3];
    let f = first(vec1);
    println!("{}", match f {
        None => { "none".to_string()}
        Some(i) => { (*i).to_string() }
    });
    vec1.push(1);
    println!("l = {}", vec1.len());
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

// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 1024];
//     stream.read(&mut buffer).unwrap();
//     let response_contents = fs::read_to_string("index.html").unwrap();
//     let response = format!(
//         "HTTP/1.1 504 OK\r\nContent-Length: {}\r\n\r\n{}",
//         response_contents.len(),
//         response_contents
//     );
//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }

// type NumberOrNothing = SomethingOrNothing<&i32>;
//
// impl NumberOrNothing {
//     fn print(self) {
//         match self {
//             Something(n) => { println!("{} is the least one", *n) }
//             Nothing => { println!("nothing") }
//         }
//     }
// }
//
// type BigIntOrNothing = SomethingOrNothing<BigInteger>;
//
// impl BigIntOrNothing {
//     fn print(self) {
//         match self {
//             Something(n) => { n.print(); }
//             Nothing => { println!("nothing") }
//         }
//     }
// }

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