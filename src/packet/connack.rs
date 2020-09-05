/// This module implements the ConnAck packet defined in MQTT spec 3.12
use crate::util::{decode_remaining_length, encode_remaining_length};

use anyhow::{anyhow, Result};

/// Connection return code
#[derive(Debug, PartialEq)]
pub enum ConnRetCode {
    Accepted,
    WrongProtocolVersion,
    IdentifierRejected,
    ServerUnavailable,
    BadUsernameOrPassword,
    NotAuthorized,
}

#[derive(Debug)]
pub struct ConnAckData {
    session_present: bool,
    ret_code: ConnRetCode,
}
/// Serialize a ConnAck packet with its data
pub fn serialize(cd: &ConnAckData) -> Vec<u8> {
    // data is everything from the variable header on
    let mut packet = Vec::<u8>::with_capacity(2);
    // 2 is for ConnAck
    packet.push(2 << 4);
    let mut variable_header = Vec::<u8>::new();
    {
        let ackflags: u8 = cd.session_present as u8;
        variable_header.push(ackflags)
    }
    variable_header.push(match cd.ret_code {
        ConnRetCode::Accepted => 0,
        ConnRetCode::WrongProtocolVersion => 1,
        ConnRetCode::IdentifierRejected => 2,
        ConnRetCode::ServerUnavailable => 3,
        ConnRetCode::BadUsernameOrPassword => 4,
        ConnRetCode::NotAuthorized => 5,
    });
    packet.extend(encode_remaining_length(variable_header.len() as u32));
    packet.extend(variable_header);
    packet
}

/// Deserialize a ConnAck packet
pub fn deserialize(pdata: &[u8]) -> Result<ConnAckData> {
    let remaining_length = decode_remaining_length(&pdata[1..])?;
    let rl_length = encode_remaining_length(remaining_length).len();
    let connack_flags = pdata[(1 + rl_length)];
    let ret_code = match pdata[(1 + rl_length + 1)] {
        0 => ConnRetCode::Accepted,
        1 => ConnRetCode::WrongProtocolVersion,
        2 => ConnRetCode::IdentifierRejected,
        3 => ConnRetCode::ServerUnavailable,
        4 => ConnRetCode::BadUsernameOrPassword,
        5 => ConnRetCode::NotAuthorized,
        _ => {
            return Err(anyhow!("Return code is malformated"));
        }
    };
    Ok(ConnAckData {
        session_present: ((connack_flags & 0x01) != 0),
        ret_code,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_success() {
        let d = ConnAckData {
            session_present: true,
            ret_code: ConnRetCode::Accepted,
        };

        let output = serialize(&d);
        let packet_type = output[0] >> 4;
        assert_eq!(2, packet_type);
        assert_eq!(2, output[1]);
        assert_eq!(0, 0b1111_1110 & output[2]);
        assert_eq!(1, 0b0000_0001 & output[2]); // session present
        assert_eq!(0, output[3]);
    }

    #[test]
    fn deserialize_success() {}

    #[test]
    fn in_and_out() {
        let din = ConnAckData {
            session_present: true,
            ret_code: ConnRetCode::Accepted,
        };
        let dout = deserialize(serialize(&din).as_slice()).unwrap();
        assert_eq!(din.session_present, dout.session_present);
        assert_eq!(din.ret_code, dout.ret_code);
    }
}
