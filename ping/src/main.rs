use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::{thread, time};
use regex::Regex;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(&mut stream);

    let re = Regex::new(r"Content-Length: (?<length>\d+)").unwrap();

    let mut line = String::new();
    let mut length: usize = 0;

    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();

    loop {
        length = match reader.read_line(&mut line) {
            Err(_) => break,
            Ok(size) if size < 3 => break,
            Ok(_) => {
                match re.captures(line.as_str()) {
                    None => 0,
                    Some(caps) => caps["length"].parse::<usize>().unwrap()
                }
            }
        }
    }

    let mut buffer = vec![0; length];
    reader.read_exact(&mut buffer).unwrap();
    let body: &str = std::str::from_utf8(&buffer).unwrap();

    println!("body = {}", body);

    thread::sleep(time::Duration::from_secs(1));

    fn respond(response: Result<&str, &str>, stream: &mut TcpStream) {
        let (status, msg) = match response {
            Ok(msg) => ("HTTP/1.1 200 OK", msg),
            Err(msg) => ("HTTP/1.1 404 NOT FOUND", msg)
        };

        let len = msg.len();
        let response = format!("{status}\r\nContent-Length: {len}\r\n\r\n{msg}");

        stream.write_all(response.as_bytes()).unwrap();
    }

    println!("req = {}", request_line);

    if request_line == "POST /pong HTTP/1.1\r\n" {
        respond(
            Ok("pong received, sending ping"),
            &mut stream
        );

        println!("sender is {}", body);

    } else {
        respond(
            Err("unrecognized request received"),
            &mut stream
        );

        println!("sender is {}", body);
    }

}