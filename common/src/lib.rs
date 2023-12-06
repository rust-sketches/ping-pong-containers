use std::{io, thread, time};
use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;

pub fn parse_http_request(reader: &mut impl io::BufRead) -> Result<(String, HashMap<String, String>, Option<String>), io::Error> {
    let mut request = String::new();
    reader.read_line(&mut request)?;

    let mut headers: HashMap<String, String> = HashMap::new();

    loop {
        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(size) if size > 2 => { // a blank line (CRLF only) separates HTTP headers and body
                match line.split_once(": ") { // HTTP headers are always formatted as "key: value"
                    Some((key, value)) => headers.insert(key.trim().into(), value.trim().into()),
                    None => continue, // skip any header lines that can't be parsed
                };
            },
            _ => break // if the reader fails to read the next line, quit early
        };
    }

    let mut body: Option<String> = None;

    if let Some(length) = headers.get("Content-Length") {
        if let Ok(length) = length.parse::<usize>() {
            let mut buffer = vec![0; length];
            reader.read_exact(&mut buffer).unwrap();
            body = Some(std::str::from_utf8(buffer.as_slice()).unwrap().into());
        }
    }

    Ok((request.trim().into(), headers, body))
}

pub fn respond(response: Result<&str, &str>, stream: &mut TcpStream) {
    let (status, msg) = match response {
        Ok(msg) => ("HTTP/1.1 200 OK", msg),
        Err(msg) => ("HTTP/1.1 404 NOT FOUND", msg)
    };

    let len = msg.len();
    let response = format!("{status}\r\nReferer: {len}\r\n\r\n{msg}");

    thread::sleep(time::Duration::from_secs(1));

    stream.write_all(response.as_bytes()).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_http_request_no_body() {
        let mut request = "\
POST /pong HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: curl/8.1.2
Accept: */*
        ";

        let (request, headers, body) = parse_http_request(&mut request.as_bytes()).unwrap();

        assert_eq!(request, "POST /pong HTTP/1.1");

        assert_eq!(headers.get("Host").unwrap(),       "127.0.0.1:7878");
        assert_eq!(headers.get("User-Agent").unwrap(), "curl/8.1.2");
        assert_eq!(headers.get("Accept").unwrap(),     "*/*");

        assert_eq!(body, None);
    }

    #[test]
    fn test_parse_http_request_with_body() {
        let mut request = "\
POST /pong HTTP/1.1
Host: 127.0.0.1:7878
User-Agent: curl/8.1.2
Accept: */*
Content-Length: 13
Content-Type: application/x-www-form-urlencoded

Hello, World!
        ";

        let (request, headers, body) = parse_http_request(&mut request.as_bytes()).unwrap();

        assert_eq!(request, "POST /pong HTTP/1.1");

        assert_eq!(headers.get("Host").unwrap(),           "127.0.0.1:7878");
        assert_eq!(headers.get("User-Agent").unwrap(),     "curl/8.1.2");
        assert_eq!(headers.get("Accept").unwrap(),         "*/*");
        assert_eq!(headers.get("Content-Length").unwrap(), "13");
        assert_eq!(headers.get("Content-Type").unwrap(),   "application/x-www-form-urlencoded");

        assert_eq!(body.unwrap(), "Hello, World!");
    }

}