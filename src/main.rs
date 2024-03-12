mod interface;
mod r#struct;
mod definition;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use crate::definition::SomethingOrNothing::{self, *, Nothing, Something};
use crate::interface::Minimum;

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

    println!("1");
    // let vec = vec![];
    let mut vec = vec![18, 5, 7, 1, 9, 27];
    vec = vec![2];
    // print_number_or_nothing(vec_min(vec));
    vec_min(&mut vec).print();
    // vec_min(&vec).print();
    // vec_min(&vec).print();
    vec.push(3);
    println!("{}", vec.len())
}

impl Minimum for i32 {
    fn compare(&self, s: Self) -> Self {
        if *self < s { *self } else { s }
    }
}
// fn print_number_or_nothing(e: SomethingOrNothing) {
//     match e {
//         Number(n) => {println!("the minimum is {}", n)}
//         Nothing() => {println!("nothing here.")}
//     }
// }

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let response_contents = fs::read_to_string("index.html").unwrap();
    let response = format!(
        "HTTP/1.1 504 OK\r\nContent-Length: {}\r\n\r\n{}",
        response_contents.len(),
        response_contents
    );
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

type NumberOrNothing = SomethingOrNothing<i32>;
impl NumberOrNothing {
    fn print(self) {
        match self {
            Something(n) => {println!("{} is the least one", n)}
            Nothing => {println!("nothing")}
        }
    }
}
/*fn vec_min(vec: Vec<i32>) -> NumberOrNothing {
    let mut min = Nothing();

    for el in vec {
        match min {
            Something(n) => {min = Something(min_of_two(el, n))}
            Something(n) => {min = Something(if el < n {el} else { n })}
            Nothing() => {min = Something(el)}
        }
    }

    min
}*/
fn vec_min<T: Minimum>(vec: &mut Vec<T>) -> SomethingOrNothing<T> {
    let mut min = Nothing;
    vec.remove(0);
    for e in vec {
        min = Something(match min {
            Something(i) => { e.compare(i) }
            Nothing => { *e }
        })
    }

    min
}
fn min_of_two(p0: i32, p1: i32) -> i32 {
    // todo!();
    if p0 < p1 { p0 } else { p1 }
}