use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

mod home;

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", http_request);

    let data = if http_request[0] == "GET / HTTP/1.1" {
        home::generate_home()
    } else {
        String::from("HTTP/1.1 404 NOT FOUND\r\n\r\n")
    };
    stream.write_all(data.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:2048").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}
