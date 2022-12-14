use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use hello::ThreadPool;

fn main() {
    eprintln!("Starting");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        eprintln!("Connection established {:?}", stream.peer_addr());

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream){
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    eprintln!("Request: {:#?}", http_request);

    let (status_line, filename) = match &http_request[0][..] {
        "GET / HTTP/1.1" => { 
            println!("Response 200");
            ("HTTP/1.1 200 OK", "hello.html")
        },
        "GET /slow HTTP/1.1" => { 
            println!("Response 200 slow");
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        },
        _ => {
            println!("Response 404");
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        },
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
