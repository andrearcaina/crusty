use std::{
    fs,
    io::{BufReader, Write, prelude::*},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use tcp_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let path = request_line.split_whitespace().nth(1).unwrap();

    let (status_line, filename) = match path {
        "/" => ("HTTP/1.1 200 OK", "hello.html"),
        "/sleep" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let full_path = format!("{}/src/{}", env!("CARGO_MANIFEST_DIR"), filename);
    let contents = fs::read_to_string(full_path).unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
