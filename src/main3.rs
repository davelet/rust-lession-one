use crate::definition::SomethingOrNothing;
use crate::r#struct::BigInteger;

mod r#struct;
mod main;
mod interface;
mod definition;

fn main() {
    // let s : SomethingOrNothing<i32> = SomethingOrNothing::Nothing;
    let s = SomethingOrNothing::Something(5_u8);
    println!("{}", s.unwrap());

    let mut bi = BigInteger::default();
    bi.print();
    bi = BigInteger::from_vec(vec![23, 4]);
    bi.print();
    bi = BigInteger::from_vec(vec![3, 6, 0]);
    bi.print();
    let data = vec![0, 1 << 7, 0, 0];
    // let p = data[3];
    // bi = BigInteger::from_vec(data);
    bi = BigInteger::from_vec(data.clone());
    bi.print();
    bi = BigInteger::from_vec(data.clone());
    // bi = BigInteger::from_vec(data);
    bi.print();
    // println!("3 ==> {}, {}", p, data.len());

    let vr = Variant::Num(8);
    work_on_variant(vr, "hello".to_string());
}

enum Variant {
    Num(i8),
    Text(String)
}

fn work_on_variant(mut var: Variant, text: String) {
    let mut ptr: &mut i8;
    match var {
        Variant::Num(ref mut n) => ptr = n,
        Variant::Text(_) => return,
    }
    /* var = Variant::Text(text); */                                /* BAD! */
    *ptr = 13;
    println!("{} {}", ptr, *ptr);
}