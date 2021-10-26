// 引入 thread 类库用来多线程处理
use std::thread;
// 引用类库 net 用来完成 TCP 监听
use std::net::{TcpListener, TcpStream, Shutdown};
// 引用 io 完成读写
use std::io::{Read, Write};

// 线程调用的处理函数
fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {
            // echo everything!
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

// 程序入口函数
fn main() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            // 当Result 枚举类型匹配Ok时
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());

                thread::spawn(move|| {
                    // connection succeeded
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                /* connection failed */
            }
        }
    }
    // close the socket server
    drop(listener);
}