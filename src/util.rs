use anyhow::{anyhow, Result};

pub fn encode_string(s: &str) -> Result<Vec<u8>> {
    if s.len() > u16::max_value() as usize {
        return Err(anyhow!("String is too long"));
    }
    let mut data = Vec::<u8>::new();
    let stringbytes = (s.len() as u16).to_be_bytes();
    data.extend(&stringbytes);
    data.extend(s.bytes());

    Ok(data)
}

pub fn decode_string(encoded: &[u8]) -> Result<(usize, String)> {
    let length_bytes = [encoded[0], encoded[1]];
    let length = u16::from_be_bytes(length_bytes);
    let lower_range_limit = 2usize;
    let upper_range_limit = (2 + length) as usize;
    let raw_string = &encoded[lower_range_limit..upper_range_limit];
    match String::from_utf8_lossy(raw_string).parse() {
        Ok(string) => Ok(((length + 2) as usize, string)),
        Err(_) => Err(anyhow!("String conversion failed")),
    }
}

/// Encode the remaining length field according to MQTT spec 2.2.3
pub fn encode_remaining_length(len: u32) -> Vec<u8> {
    let mut data = Vec::<u8>::new();
    let mut temp = len;

    loop {
        let mut encoded = (temp % 128) as u8;
        temp /= 128;

        if temp > 0 {
            encoded |= 0x80;
        }
        data.push(encoded);
        if temp == 0 {
            break;
        }
    }

    if data.is_empty() {
        data.push(0);
    }

    data
}

/// Decode the remaining length field according to MQTT spec 2.2.3
pub fn decode_remaining_length(encoded: &[u8]) -> Result<u32> {
    let mut multiplier: u32 = 1;
    let mut value: u32 = 0;
    let mut inputiter = encoded.iter();
    loop {
        let encoded_byte = match inputiter.next() {
            Some(b) => (*b) as u32,
            None => {
                return Err(anyhow!("Could not fetch enough bytes"));
            }
        };
        value += (encoded_byte & 0x7F) * multiplier;
        multiplier *= 128;
        if (encoded_byte & 0x80) == 0 {
            break;
        }

        if multiplier > ((0x80 * 0x80 * 0x80) as u32) {
            return Err(anyhow!("Malformed remaining length"));
        }
    }
    Ok(value)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn encode_string_success() {
        let s = String::from("Hello World!");
        let v = encode_string(&s).unwrap();
        assert_eq!(14, v.len());
        assert_eq!(0, v[0]);
        assert_eq!(12, v[1]);
    }

    #[test]
    fn encode_remaining_length_success() {
        {
            let rl = encode_remaining_length(5);
            assert_eq!(1, rl.len());
            assert_eq!(5, rl[0]);
        }
        {
            let rl = encode_remaining_length(128);
            assert_eq!(2, rl.len());
            assert_eq!(0x80, rl[0]);
            assert_eq!(0x01, rl[1]);
        }
        {
            let rl = encode_remaining_length(16384);
            assert_eq!(3, rl.len());
            assert_eq!(0x80, rl[0]);
            assert_eq!(0x80, rl[1]);
            assert_eq!(0x01, rl[2]);
        }
    }

    #[test]
    fn decode_remaining_length_success() {
        {
            let input_length = 25;
            let input = vec![25u8];
            let length = decode_remaining_length(input.as_slice()).unwrap();
            assert_eq!(input_length, length);
        }
        {
            let input_length = 128;
            let input = vec![0x80, 0x01];
            let length = decode_remaining_length(input.as_slice()).unwrap();
            assert_eq!(input_length, length);
        }
        {
            let input_length = 16384;
            let input = vec![0x80, 0x80, 0x01];
            let length = decode_remaining_length(input.as_slice()).unwrap();
            assert_eq!(input_length, length);
        }
        {
            let input_length = 2097152;
            let input = vec![0x80, 0x80, 0x80, 0x01];
            let length = decode_remaining_length(input.as_slice()).unwrap();
            assert_eq!(input_length, length);
        }
        {
            let input_length = 2097151;
            let input = vec![0xFF, 0xFF, 0x7F];
            let length = decode_remaining_length(input.as_slice()).unwrap();
            assert_eq!(input_length, length);
        }
    }
}
