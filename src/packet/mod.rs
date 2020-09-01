use bytes::{BytesMut, BufMut, Bytes, Buf};
use std::error::Error;

pub trait MqttPacket {
    fn fill_buffer(self: &Self, buf: &mut BytesMut);
}

fn encode_string(s: String) -> Result<Vec<u8>, &str> {
    if s.len() > u16::max_value() as usize {
        return Err("String is too long");
    }
    let mut data = Vec::<u8>::new();
    let stringbytes = (s.len() as u16).to_be_bytes();
    data.extend(&stringbytes);
    data.extend(s.bytes());

    return Ok(data);
}

pub struct Will {
    topic: String,
    message: String,
    qos: u8,
    retain: bool,
}

pub struct Connect {
    pub keep_alive: u16,
    pub client_ident: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub clean_session: bool,
    pub will: Option<Will>,
}

impl MqttPacket for Connect {
    fn fill_buffer(self: &Connect, buf: &mut BytesMut) {
        let mut length: usize = 10;
        length += self.client_ident.len() + 2;
        match &self.username {
            Some(username) => {
                length += username.len() + 2;
                match &self.password {
                    Some(password) => {
                        length += password.len() + 2;
                    }
                    None => {}
                }
            }
            None => {}
        }
        match &self.will {
            Some(will) => {
                length += will.topic.len() + 2;
                length += will.message.len() + 2;
            }
            None => {}
        }

        buf.put_u8((1u8 << 4) as u8);
        buf.put_u8(length as u8);
        buf.put_u16(4);
        buf.put(&b"MQTT"[..]);
        buf.put_u8(4);

        let mut connect_flags: u8 = 0;
        if self.clean_session {
            connect_flags |= 1 << 1;
        }
        match &self.username {
            Some(_) => {
                connect_flags |= 1 << 7;
                match &self.password {
                    Some(_) => {
                        connect_flags |= 1 << 6;
                    }
                    None => {}
                }
            }
            None => {}
        }
        match &self.will {
            Some(will) => {
                connect_flags |= 1 << 2;
                if will.retain {
                    connect_flags |= 1 << 5;
                    connect_flags |= (will.qos & 0x03) << 3;
                }
            }
            None => {}
        }
        buf.put_u8(connect_flags);
        buf.put_u16(self.keep_alive);
        buf.put_u16(self.client_ident.len() as u16);
        buf.put(self.client_ident.as_bytes());
        // buf.put_u16(5);
        match &self.will {
            Some(will) => {
                buf.put_u16(will.topic.len() as u16);
                buf.put(will.topic.as_bytes());
                buf.put_u16(will.message.len() as u16);
                buf.put(will.message.as_bytes());
            }
            None => {}
        }
        match &self.username {
            Some(username) => {
                buf.put_u16(username.len() as u16);
                buf.put(username.as_bytes());
            }
            None => {}
        }
        match &self.password {
            Some(password) => {
                buf.put_u16(password.len() as u16);
                buf.put(password.as_bytes());
            }
            None => {}
        }
    }
}

mod test {
    use super::*;

    #[test]
    fn encode_string_success() {
        let s = String::from("Hello World!");
        let v = encode_string(s);
        assert_eq!(14, v.len());
        assert_eq!(0, v[0]);
        assert_eq!(12, v[1]);
    }
}
