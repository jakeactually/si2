pub fn uncompress(data: Vec<u8>) -> Vec<u8> {
    let mut result = vec![];

    for byte in data.iter() {
        for i in 0..8 {
            result.push((byte >> (7 - i)) & 1);
        }
    }

    result
}
