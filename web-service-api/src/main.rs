use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use redis::Commands;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("starting server...");

    let _arr: [i32; 20] = [2; 20];
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
        println!("do_something...");
        match do_something() {
            Ok(_) => {
                println!("OK!");
            }
            Err(_) => {
                println!("failed!");
            }
        };
    }
}

fn do_something() -> redis::RedisResult<()> {
    println!("entering do_something...");
    // TODO: this needs to be read from config or using kube services
    // TODO: can we reuse the connection instead of open a connection every time?
    let client = redis::Client::open("redis://0.0.0.0:6379/")?;
    let mut con = client.get_connection()?;

    println!("established connection...");
    let _: () = con.set("my_key", 42)?;

    println!("after set my_key");
    let val: isize = con.get("my_key")?;

    println!("{}", val);
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    println!("handling connection");
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
