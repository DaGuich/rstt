use anyhow::{anyhow, Result};

pub fn serialize() -> Vec<u8> {
    let mut packet = Vec::<u8>::with_capacity(2);
    packet.push(12 << 4);
    packet.push(0);
    packet
}

pub fn deserialize(pdata: &[u8]) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_success() {
    }

    #[test]
    fn deserialize_success() {}

    #[test]
    fn in_and_out() {
    }
}
