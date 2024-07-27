
use std::{ env, fs::{self, File}, io::{Read, Write}, net::{TcpListener, TcpStream}, thread};

fn callback(mut stream: TcpStream) {
    let path = env::current_dir().unwrap().join("log").join(stream.peer_addr().unwrap().to_string().replace(":", "-")).to_str().unwrap().to_owned();
    println!("{}",path);
    let mut file = File::options().write(true).create(true).open(path+".txt").unwrap();
    loop {
        let mut buffer =[0;512];
        match stream.read(&mut buffer) {
            Ok(size) => {
                if size != 0{
                    file.write_all(&buffer[..size]).unwrap();
                    file.flush().unwrap();
                    
                } 
                else if size == 0  {
                    stream.shutdown(std::net::Shutdown::Both).unwrap();
                }
            },
            Err(e) => {println!("some error occured {}",e);
                                stream.shutdown(std::net::Shutdown::Both).unwrap();}
        }
    }
    
}

fn main() {
    match fs::create_dir(env::current_dir().unwrap().join("log")){
        Ok(_) => {},
        Err(_) => {}
    }
    match TcpListener::bind("127.0.0.1:6000"){
        Ok( listner) => {
            for stream in listner.incoming() {
                match stream{
                    Ok( stream) => {
                        println!("{}",stream.peer_addr().unwrap().to_string());
                        thread::spawn(move|| callback(stream));
 
                    },
                    Err(e) => println!("fail to establish connection due to : {}",e)
                }
            }
        }
        Err(e) => println!("Fail to start server : {}",e)
    }
    
}