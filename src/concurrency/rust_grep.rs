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
                                Ok(ok) => {println!("READ send = {} >> {}", i, line);
                                    i += 1
                                }
                                Err(err) => {println!("sent line err :{}", err)}
                            }
                        }
                        Err(err) => {println!("read line err :{}", err)}
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
            vec.sort();
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
        files: vec![r#"src/main1.rs"#.to_string()],
        patter: "m".to_string(),
        mode: OutputMode::CountToPrint,
    };
    execute(o);
    println!("finished")
}