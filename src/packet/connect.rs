use anyhow::{anyhow, Result};

use crate::util::{decode_remaining_length, decode_string, encode_remaining_length, encode_string};

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

pub fn deserialize(pdata: &[u8]) -> Result<ConnectData> {
    let remaining_length = match decode_remaining_length(&pdata[1..]) {
        Ok(d) => d,
        Err(e) => return Err(e),
    };
    let remaining_length_len = encode_remaining_length(remaining_length).len();

    let pdata = &pdata[(1 + remaining_length_len)..];

    let mut jump_size = 0usize;
    {
        let (plen, pname) = match decode_string(pdata) {
            Ok((len, name)) => (len, name),
            Err(e) => {
                return Err(e);
            }
        };
        jump_size = plen;

        if pname != "MQTT" {
            return Err(anyhow!("Protocol name is wrong"));
        }
    }
    let pdata = &pdata[jump_size..];
    jump_size = 0;

    {
        let protocol_version = pdata[0];
        jump_size += 1;
        if protocol_version != 4 {
            return Err(anyhow!(
                "Protocol version not correct. Only 3.1.1 supported"
            ));
        }
    }
    let connect_flags = pdata[1];
    jump_size += 1;
    let clean_session = ((1 << 1) & connect_flags) != 0;
    let will_flag = ((1 << 2) & connect_flags) != 0;
    let will_qos = 0b0000_0011 & (connect_flags >> 3);
    let will_retain_flag = ((1 << 5) & connect_flags) != 0;
    let username_flag = ((1 << 7) & connect_flags) != 0;
    let password_flag = ((1 << 6) & connect_flags) != 0;

    let pdata = &pdata[jump_size..];
    jump_size = 0;
    let keep_alive = {
        let bytes = [pdata[0], pdata[1]];
        jump_size += 2;
        u16::from_be_bytes(bytes)
    };
    let pdata = &pdata[jump_size..];
    jump_size = 0;

    Ok(ConnectData {
        keep_alive,
        clean_session,
        client_ident: String::from("abc"),
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
