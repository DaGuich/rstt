use std::net::TcpStream;
use std::io::{Write, Read};
use bytes::{BytesMut,BufMut};
use crate::packet::MqttPacket;
use std::net::Shutdown::Both;

mod packet;


fn main() {
    // [0x10, 15, 0, 4, 0x4D, 0x51, 0x54, 0x54, 4, 0, 0, 60, 0, 3, 0x57, 0x57, 0x57];
    let mut buffer = BytesMut::with_capacity(1024);
    let mut stream = TcpStream::connect("127.0.0.1:1883").unwrap();
    let pack = packet::Connect{
        keep_alive: 60,
        client_ident: String::from("hilfe"),
        username: None,
        password: None,
        // username: Some(String::from("matthias")),
        // password: Some(String::from("pw")),
        clean_session: true,
        will: None
    };
    pack.fill_buffer(&mut buffer);
    println!("Buffer: {:X?}", buffer.to_vec());
    let nbytes = stream.write(buffer.as_ref()).unwrap();
    println!("Sent {} bytes!", nbytes);
    stream.shutdown(Both).unwrap();
    println!("Shutdown");
}

