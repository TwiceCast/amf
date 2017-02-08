extern crate amf;
extern crate byteorder;
extern crate serde;

use amf::{Deserializer, SliceReader, Value};
use std::io::{Read, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::{str};
use serde::Deserialize;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5432").unwrap();

    let mut stream = listener.accept().unwrap().0;
    stream.write_all("end".as_bytes()).unwrap();
    handle_request(stream);
}

fn handle_request(stream: TcpStream) {
    let mut reader = BufReader::new(stream);
    let mut array = [0; 128 ]; //Never forget to augment this size
    let _ = reader.read(&mut array).unwrap();
/*    for nb in array.iter() {
    	println!("{}", nb);
    }*/
    let s = SliceReader::new(&array);
    let mut de = Deserializer{reader: s};
    let v = Value::deserialize(&mut de).unwrap();
    println!("{}", v);
}
