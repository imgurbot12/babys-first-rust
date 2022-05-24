mod http;


use std::io::{Write, BufWriter, BufReader, BufRead};
use std::collections::HashMap;

fn main() {
    let mut req = http::Request{
        method:  "GET".to_string(),
        path:    "/index.html".to_string(),
        version: "1.0".to_string(),
        headers: HashMap::new(),
    };
    req.headers.insert("Ayy".to_string(), "Lmao".to_string());
    req.headers.insert("Host".to_string(), "127.0.0.1".to_string());

    let mut buf = BufWriter::new(Vec::new());
    req.write_reqline(&mut buf as &mut dyn Write).unwrap();
    req.write_headers(&mut buf as &mut dyn Write).unwrap();

    let raw = String::from_utf8_lossy(buf.buffer());

    let mut body = BufReader::new(raw.as_bytes());
    let mut req2 = http::Request::new();
    req2.parse_reqline(&mut body as &mut dyn BufRead).unwrap();
    req2.parse_headers(&mut body as &mut dyn BufRead).unwrap();
    println!("{:?}", raw);
    println!("{:#?}", req2)

    // println!("Hello, world!");
}
