// use std::io::prelude::*;
// use std::net::TcpListener;
// use std::net::TcpStream;

// use std::fs;
use rand::Rng;
use redis::Commands;
use std::time::Instant;
// use std::time::{Duration, Instant};

pub fn main() {
    // let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    // println!("starting server...");

    // let _arr: [i32; 20] = [2; 20];
    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     handle_connection(stream);
    //     println!("do_something...");
    //     match do_something() {
    //         Ok(_) => {
    //             println!("OK!");
    //         }
    //         Err(_) => {
    //             println!("failed!");
    //         }
    //     };
    // }

    let now = Instant::now();
    for i in 0..1000 {
        match do_something() {
            Ok(_) => {
            }
            Err(_) => {
                println!("failed! {}", i);
            }
        };
    }

    println!("loop done in {}ms", now.elapsed().as_millis());

    let now = Instant::now();
    let mut con = redis::Client::open("redis://0.0.0.0:6379/")
        .expect("client creation failed")
        .get_connection()
        .expect("get conn failed");

    let mut rng = rand::thread_rng();
    for _ in 0..1000 {
        let n1 = rng.gen::<u32>().to_string();
        let n2 = rng.gen::<u32>().to_string();
        let _: () = con.set(n1, n2).unwrap();

    }

    println!("loop 2 done in {}ms", now.elapsed().as_millis());
}

fn do_something() -> redis::RedisResult<()> {
    // TODO: this needs to be read from config or using kube services
    // TODO: can we reuse the connection instead of open a connection every time?
    let client = redis::Client::open("redis://0.0.0.0:6379/")?;
    let mut con = client.get_connection()?;

    let mut rng = rand::thread_rng();

    let n1 = rng.gen::<u32>().to_string();
    let n2 = rng.gen::<u32>().to_string();
    let _: () = con.set(n1, n2)?;

    // let val: isize = con.get("my_key")?;

    // println!("{}", val);
    Ok(())
}

// fn handle_connection(mut stream: TcpStream) {
//     let mut buffer = [0; 512];
//     println!("handling connection");
//     stream.read(&mut buffer).unwrap();

//     let contents = fs::read_to_string("hello.html").unwrap();

//     let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

//     stream.write(response.as_bytes()).unwrap();
//     stream.flush().unwrap();
// }
