mod connack;
mod connect;
mod pingreq;
mod pingresp;
mod disconnect;

use connack::ConnAckData;
use connect::ConnectData;

use anyhow::{anyhow, Result};

pub enum PType {
    Connect(ConnectData),
    ConnAck(ConnAckData),
    PingReq,
    PingResp,
    Disconnect,
}

impl From<&[u8]> for PType {
    fn from(pdata: &[u8]) -> Self {
        let t = deserialize(&pdata).unwrap();
        t
    }
}

pub fn serialize(packet: PType) -> Vec<u8> {
    let mut pbytes = Vec::<u8>::new();
    match packet {
        PType::Connect(data) => {
            pbytes.extend(connect::serialize(&data));
        }
        PType::ConnAck(data) => {
            pbytes.extend(connack::serialize(&data));
        },
        PType::PingReq => {
            pbytes.extend(pingreq::serialize());
        },
        PType::PingResp => {
            pbytes.extend(pingresp::serialize());
        },
        PType::Disconnect => {
            pbytes.extend(disconnect::serialize());
        }
    }
    pbytes
}

pub fn deserialize(pbytes: &[u8]) -> Result<PType> {
    let packettype = pbytes[0] >> 4;
    match packettype {
        1 => {
            let cd = connect::deserialize(&pbytes);
            match cd {
                Ok(data) => Ok(PType::Connect(data)),
                Err(e) => Err(e),
            }
        }
        2 => {
            let cd = connack::deserialize(&pbytes);
            match cd {
                Ok(data) => Ok(PType::ConnAck(data)),
                Err(e) => Err(e),
            }
        },
        12 => {
            let res = pingreq::deserialize(&pbytes);
            match res {
                Ok(_) => Ok(PType::PingReq),
                Err(e) => Err(e),
            }
        },
        13 => {
            let res = pingresp::deserialize(&pbytes);
            match res {
                Ok(_) => Ok(PType::PingResp),
                Err(e) => Err(e),
            }
        },
        14 => {
            let res = disconnect::deserialize(&pbytes);
            match res {
                Ok(_) => Ok(PType::Disconnect),
                Err(e) => Err(e),
            }
        },
        _ => Err(anyhow!("Packet not valid")),
    }
}
