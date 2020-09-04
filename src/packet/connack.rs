use crate::util::encode_remaining_length;

use anyhow::{anyhow, Result};

pub enum ConnRetCode {
    Accepted,
    WrongProtocolVersion,
    IdentifierRejected,
    ServerUnavailable,
    BadUsernameOrPassword,
    NotAuthorized,
}

pub struct ConnAckData {
    session_present: bool,
    ret_code: ConnRetCode,
}

pub fn serialize(cd: &ConnAckData) -> Vec<u8> {
    // data is everything from the variable header on
    let mut packet = Vec::<u8>::with_capacity(2);
    // 2 is for ConnAck
    packet.push(2 << 4);
    let mut variable_header = Vec::<u8>::new();
    {
        let ackflags: u8 = (cd.session_present as u8) << 1;
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

pub fn deserialize(_pdata: &[u8]) -> Result<ConnAckData> {
    unimplemented!("Connect packet deserialization not implemented yet");
}
