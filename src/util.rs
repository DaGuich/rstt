pub fn encode_string(s: &str) -> Result<Vec<u8>, &'static str> {
    if s.len() > u16::max_value() as usize {
        return Err("String is too long");
    }
    let mut data = Vec::<u8>::new();
    let stringbytes = (s.len() as u16).to_be_bytes();
    data.extend(&stringbytes);
    data.extend(s.bytes());

    Ok(data)
}

pub fn encode_remaining_length(len: u32) -> Vec<u8> {
    let mut data = Vec::<u8>::new();
    let mut temp = len;

    while temp > 0 {
        let mut encoded = (temp % 128) as u8;
        temp /= 128;

        if temp > 0 {
            encoded |= 0x80;
        }
        data.push(encoded);
    }

    if data.is_empty() {
        data.push(0);
    }

    data
}

pub fn decode_remaining_length(encoded: &[u8]) -> Result<u32, &'static str> {
    let mut multiplier: u32 = 1;
    let mut value: u32 = 0;
    let mut inputiter = encoded.iter();
    loop {
        let encoded_byte = *(inputiter.next().unwrap()) as u32;
        value += (encoded_byte & 127u32) * multiplier;
        if multiplier > ((128 * 128 * 128) as u32) {
            return Err("Malformed remaining length");
        }
        if (encoded_byte & 128) != 0 {
            break;
        }
        multiplier *= 128;
    }
    Ok(value)
}

#[cfg(test)]
mod test {
    use super::{encode_remaining_length, encode_string};

    #[test]
    fn encode_string_success() {
        let s = String::from("Hello World!");
        let v = encode_string(&s).unwrap();
        assert_eq!(14, v.len());
        assert_eq!(0, v[0]);
        assert_eq!(12, v[1]);
    }

    #[test]
    fn remaining_length_success() {
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
}
