use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use web_server::*;
use std::env;

/// run simple multi-threading HTTP server
fn main() {
    // receive number of threads from user
    let args: Vec<String> = env::args().collect();
    
    match args.len() {
        2 => (),
        _ => panic!("need only one argument 'num_of_threads'")
    };
    
    let num_of_threads : usize = args[1].parse::<usize>().expect("please enter numbers only");
    println!("Starting sever with {} threads...", num_of_threads);

    let listener = TcpListener::bind("127.0.0.1:7878").expect("bind had falied");

    let pool = match ThreadPool::new(num_of_threads) {
        Ok(pool)  => pool,
        Err(_) => panic!("Error had occured during ThreadPool::new")
    };

    for stream in listener.incoming() {
        let stream = stream.expect("connection attemp had falied");
        
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

/// handle incoming connection
/// 
/// return simple HTTP answer, support multi-threading
fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("stream read had falied");

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
