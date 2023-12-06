use std::io::BufReader;
use std::net::{TcpListener, TcpStream};

use common::{parse_http_request, respond};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut reader = BufReader::new(&mut stream);

    match parse_http_request((&mut reader).into()) {
        Ok((request, headers, body)) => {
            if request == "POST /pong HTTP/1.1" {
                respond(
                    Ok("pong received, sending ping"),
                    &mut stream
                );

                println!("sender is {}", body.unwrap_or(String::from("unknown")));

            } else {
                respond(
                    Err("unrecognized request received"),
                    &mut stream
                );

                println!("sender is {}", body.unwrap_or(String::from("unknown")));
            }
        },
        Err(_) => println!("Could not parse http request")
    }

}