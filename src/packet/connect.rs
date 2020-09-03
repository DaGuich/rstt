use crate::util::{encode_remaining_length, encode_string};

pub struct ConnectWill {
    topic: String,
    message: String,
    qos: u8,
    retain: bool,
}

pub struct ConnectData {
    pub keep_alive: u16,
    pub client_ident: String,
    pub username: Option<String>,
    pub password: Option<String>,
    pub clean_session: bool,
    pub will: Option<ConnectWill>,
}

pub fn serialize(cd: &ConnectData) -> Vec<u8> {
    // data is everything from the variable header on
    let mut variable_header = Vec::<u8>::new();
    let mut payload = Vec::<u8>::new();

    // MQTT header
    variable_header.extend(encode_string(&String::from("MQTT")).unwrap());
    // protocol version 4 = 3.1.1
    variable_header.push(4);

    let mut connect_flags = 0u8;
    if cd.clean_session {
        connect_flags |= 1 << 1;
    }

    payload.extend(encode_string(&cd.client_ident).unwrap());

    match &cd.will {
        Some(will) => {
            connect_flags |= 1 << 2;
            connect_flags |= (will.qos & 0x03) << 3;
            if will.retain {
                connect_flags |= 1 << 5;
            }
            payload.extend(encode_string(&will.topic).unwrap());
            payload.extend(encode_string(&will.message).unwrap());
        }
        None => {}
    }

    match &cd.username {
        Some(username) => {
            connect_flags |= 1 << 7;
            payload.extend(encode_string(username).unwrap());
            match &cd.password {
                Some(password) => {
                    connect_flags |= 1 << 6;
                    payload.extend(encode_string(password).unwrap());
                }
                None => {}
            }
        }
        None => {}
    }

    variable_header.push(connect_flags);

    let mut fixed_header = Vec::<u8>::new();
    let encoded_len = encode_remaining_length(variable_header.len() as u32);
    fixed_header.push(1 << 1);
    fixed_header.extend(encoded_len);

    let mut data = Vec::<u8>::new();
    data.extend(fixed_header);
    data.extend(variable_header);
    data.extend(cd.keep_alive.to_be_bytes().iter());
    data.extend(payload);
    data
}

pub fn deserialize(_pdata: &[u8]) -> Result<ConnectData, &'static str> {
    unimplemented!("Connect packet deserialization not implemented yet");
}
