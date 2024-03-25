use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::mpsc::{Receiver, sync_channel, SyncSender};
use std::thread;
use std::time::Duration;

fn produce(file: String, channel: SyncSender<String>) {
    let file = File::open(file);
    match file {
        Ok(file) => {
            let file = BufReader::new(file);
            for line in file.lines() {
                match line {
                    Ok(line) => {
                        let send_rs = channel.send(line.clone());
                        match send_rs {
                            Ok(ok) => {}
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

fn consume(channel: Receiver<String>) {
    let mut vec: Vec<String> = channel.iter().collect();
    let v = &mut vec[..];
    vec.sort();
    for (i, line) in vec.iter().enumerate() {
        println!("{}: {}", i, line)
    }
}

#[test]
fn p_c() {
    let file = r#"src/main.rs"#.to_string();
    let (sender, receiver) = sync_channel(10);
    thread::spawn(|| produce(file, sender));
    thread::spawn(|| consume(receiver));

    thread::sleep(Duration::from_secs(5));
}