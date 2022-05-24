use std::io::{BufRead, Write, Error, ErrorKind};
use std::collections::HashMap;

/* Functions */

#[inline]
fn eof(s: &str) -> Error {
    Error::new(ErrorKind::UnexpectedEof, s)
}

#[inline]
fn errdata(s: &str) -> Error {
    Error::new(ErrorKind::InvalidData, s)
}

/* Classes */

#[derive(Debug,Default)]
pub struct Request {
    pub method:  String,
    pub path:    String,
    pub version: String,
    pub headers: HashMap<String, String>,
}

impl Request {

    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse_reqline(&mut self, reader: &mut dyn BufRead) -> Result<(), Error> {
        // read request line from reader
        let mut line = String::new();
        reader.read_line(&mut line)?;
        // parse line into method/path/version
        let v: Vec<&str> = line.split_terminator(' ').collect();
        if v.len() != 3 {
            return Err(eof("invalid request line"));
        }
        self.method = v[0].to_string();
        self.path   = v[1].to_string();
        match v[2].rsplit_once("/") {
            Some((_, version)) => self.version = version.trim_end().to_string(),
            None => return Err(eof("invalid http version"))
        }
        Ok(())
    }

    pub fn parse_headers(&mut self, reader: &mut dyn BufRead) -> Result<(), Error> {
        loop {
            // read next line from buffer
            let mut line = String::new();
            reader.read_line(&mut line)?;
            if line.trim_end().is_empty() {
                return Ok(());
            }
            // split into key/value
            match line.split_once(": ") {
                Some((k, v)) => self.headers.insert(k.to_string(), v.trim_end().to_string()),
                None => return Err(eof("invalid header line")),
            };
        }
    }

    pub fn write_reqline(&self, writer: &mut dyn Write) -> Result<(), Error> {
        // raise error if any of the required fields are missing
        if self.method.is_empty() || self.path.is_empty() || self.version.is_empty() {
            return Err(errdata("method/path/version must be specified"));
        }
        // write new request line to writer
        let fmt = format!("{} {} HTTP/{}\r\n", self.method.to_uppercase(), self.path, self.version);
        writer.write(fmt.as_bytes())?;
        Ok(())
    }

    pub fn write_headers(&self, writer: &mut dyn Write) -> Result<(), Error> {
        for (key, value) in self.headers.iter() {
            writer.write(format!("{key}: {value}\r\n").as_bytes())?;
        }
        writer.write("\r\n".as_bytes())?;
        Ok(())
    }
}
