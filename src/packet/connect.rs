use crate::util::{decode_remaining_length, encode_remaining_length, encode_string};

#[derive(Debug)]
pub struct ConnectWill {
    topic: String,
    message: String,
    qos: u8,
    retain: bool,
}

#[derive(Debug)]
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

pub fn deserialize(pdata: &[u8]) -> Result<ConnectData, &'static str> {
    let remaining_length = match decode_remaining_length(&pdata[1..]) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    let remaining_length_len = encode_remaining_length(remaining_length).len();

    let pdata = &pdata[(2 + remaining_length_len)..];

    {
        let protocol_name_length_bytes: [u8; 2] = [pdata[0], pdata[1]];

        if u16::from_be_bytes(protocol_name_length_bytes) != 4 {
            return Err("Protocol name not in correct format");
        }
        let protocol_name = String::from_utf8_lossy(&pdata[2..5]);
        if protocol_name != "MQTT" {
            return Err("Protocol name not correct");
        }
    }

    let pdata = &pdata[6..];

    {
        let protocol_version = pdata[0];
        if protocol_version != 4 {
            return Err("Protocol version not correct. Only 3.1.1 supported");
        }
    }

    // let pdata = &pdata[1..];

    Ok(ConnectData {
        keep_alive: 0,
        client_ident: String::from("abc"),
        clean_session: false,
        username: None,
        password: None,
        will: None,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn in_and_out_success() {
        let data_in = ConnectData {
            keep_alive: 60,
            client_ident: String::from("abc"),
            clean_session: false,
            username: None,
            password: None,
            will: None,
        };
        let data_out = deserialize(serialize(&data_in).as_mut_slice()).unwrap();
        assert_eq!(data_in.keep_alive, data_out.keep_alive);
        assert_eq!(data_in.client_ident, data_out.client_ident);
        assert_eq!(data_in.clean_session, data_out.clean_session);
        assert_eq!(data_in.username, data_out.username);
        assert_eq!(data_in.password, data_out.password);
        assert_eq!(data_in.will.is_none(), data_out.will.is_none());
    }
}
