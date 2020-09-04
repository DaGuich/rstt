mod connack;
mod connect;

use connack::ConnAckData;
use connect::ConnectData;

pub enum PType {
    Connect(ConnectData),
    ConnAck(ConnAckData),
}

pub fn serialize(packet: PType) -> Vec<u8> {
    let mut pbytes = Vec::<u8>::new();
    match packet {
        PType::Connect(data) => {
            pbytes.extend(connect::serialize(&data));
        }
        PType::ConnAck(data) => {
            pbytes.extend(connack::serialize(&data));
        }
    }
    pbytes
}

pub fn deserialize(pbytes: &[u8]) -> Result<PType, &'static str> {
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
                Err(e) => Err(e)
            }
        }
        _ => Err("Packet not valid"),
    }
}
