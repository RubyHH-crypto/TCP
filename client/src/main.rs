// 引用 net 用来完成 TCP 监听
use std::net::{TcpStream};
// 引入 io 进行读取
use std::io::{Read, Write};
// 引入 str 转换输入为 str 类型
use std::str::from_utf8;

//主程序入口
fn main() {
    // 定义请求地址 IP：端口
    match TcpStream::connect("localhost:3333") {
        //匹配模式
        Ok(mut stream) => {
            // 取得链接时 OK
            println!("We got you, Successfully connected to server in port 3333");

            let msg = b"Hello!";
            // 打印互动信息
            stream.write(msg).unwrap();
            println!("Sent Hello, awaiting reply...");
            // 获取切片数组
            let mut data = [0 as u8; 6]; // using 6 byte buffer
            // 取得回复信息时
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    // 获取正确信息
                    if &data == msg {
                        // 获取输入
                        let mut input = String::new();
                        
                        println!("Enter your reply :");
                        // 打印输入信息
                        let reply=std::io::stdin().read_line(&mut input).unwrap();
                        println!("no of bytes read , {}", reply);
                        
                    // 获取异常信息
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    // 未收到信息
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            // 链接错误时
            println!("Failed to connect: {}", e);
        }
    }
    //请求终止
    println!("Terminated.");
}
