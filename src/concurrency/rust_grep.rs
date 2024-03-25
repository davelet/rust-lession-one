use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use std::sync::mpsc::{Receiver, SendError, sync_channel, SyncSender};
use std::thread;

#[derive(Copy, Clone)]
pub enum OutputMode {
    Print,
    SortAndPrint,
    CountToPrint,

}

pub struct Options {
    pub files: Vec<String>,
    pub patter: String,
    pub mode: OutputMode,
}

pub fn read_files(options: Arc<Options>, channel: SyncSender<String>) {
    let mut i = 0_u16;
    for file in options.files.iter() {
        let file = File::open(file);
        match file {
            Ok(file) => {
                let file = BufReader::new(file);
                for line in file.lines() {
                    match line {
                        Ok(line) => {
                            let send_rs = channel.send(line.clone());
                            match send_rs {
                                Ok(ok) => {
                                    println!("READ send = {} >> {}", i, line);
                                    i += 1
                                }
                                Err(err) => { println!("sent line err :{}", err) }
                            }
                        }
                        Err(err) => { println!("read line err :{}", err) }
                    }
                }
            }
            Err(err) => { println!("read file err: {}", err) }
        }
    }
}

pub fn match_lines(options: Arc<Options>, in_channel: Receiver<String>, out_channel: SyncSender<String>) {
    let mut i = 0_u32;
    for line in in_channel.iter() {
        println!("GOT LINE = {}  >> {}", i, line);
        if line.contains(&options.patter) {
            out_channel.send(line).unwrap()
        }
        i += 1;
    }
}

pub fn output_matched_result(options: Arc<Options>, channel: Receiver<String>) {
    match options.mode {
        OutputMode::Print => {
            for line in channel.iter() {
                println!("{}", line)
            }
        }
        OutputMode::SortAndPrint => {
            let mut vec: Vec<String> = channel.iter().collect();
            let v = &mut vec[..];
            // vec.sort();
            sort(&mut vec[..]);
            for (i, line) in vec.iter().enumerate() {
                println!("{}: {}", i, line)
            }
        }
        OutputMode::CountToPrint => {
            let count = channel.iter().count();
            println!("{} of {}", count, options.patter)
        }
    }
}

fn sort<T: PartialOrd>(list: &mut [T]) {
    if list.len() < 2 { return; } // 递归出口

    let mut lpos = 1;
    let mut rpos = list.len() - 1;

    loop {
        if lpos > rpos { break; }
        if list[0] >= list[lpos] {
            lpos += 1
        } else if list[0] < list[lpos] {
            list.swap(lpos, rpos);
            rpos -= 1;
        }
    }
    list.swap(0, lpos - 1);
    let parts = list.split_at_mut(lpos);
    sort(&mut parts.0[..lpos - 1]);
    sort(parts.1);
}

#[test]
fn t() {
    // let mut v = [5,4,3,2,1,9];
    let mut v = ["a", "c", "b"];
    let mut a = v;
    println!("{}", a[0]);
    sort(&mut v);
    println!("{:?}", v);
    a[0] = "z";
    println!("{:?}", a);
    println!("{:?}", v);

    let mut array_of_data: [f64; 5] = [1.0, 3.4, 12.7, -9.12, 0.1];
    sort(&mut array_of_data[..]);
    println!("{:?}", array_of_data);
}

pub fn execute(options: Options) {
    let options = Arc::new(options);

    let line_channel = sync_channel(4);
    let option0 = options.clone();
    let runnable0 = thread::spawn(|| read_files(option0, line_channel.0));

    let option1 = options.clone();
    let match_channel = sync_channel(4);
    let runnable1 = thread::spawn(|| match_lines(option1, line_channel.1, match_channel.0));

    let option2 = options.clone();
    let runnable2 = thread::spawn(|| output_matched_result(option2, match_channel.1));

    runnable0.join().unwrap();
    runnable1.join().unwrap();
    runnable2.join().unwrap();
}

#[test]
fn test() {
    let o = Options {
        files: vec![r#"src/main.rs"#.to_string()],
        patter: "".to_string(),
        mode: OutputMode::SortAndPrint,
    };
    execute(o);
    println!("finished")
}